use esp_hal::Blocking;
use esp_hal::i2c::master::I2c;
use esp_hal::time::{Duration, Instant};
use log::info;

const DDC_ADDR: u8 = 0x37;
const EDID_ADDR: u8 = 0x50;
const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

macro_rules! bail { ($($t:tt)*) => {{ info!($($t)*); return false; }} }

pub struct VcpController<'a> {
    i2c: I2c<'a, Blocking>,
}

impl<'a> VcpController<'a> {
    pub fn new(i2c: I2c<'a, Blocking>) -> Self {
        Self { i2c }
    }
    fn delay_us(micros: u64) {
        let start = Instant::now();
        while start.elapsed() < Duration::from_micros(micros) {}
    }
    fn checksum(addr: u8, data: &[u8]) -> u8 {
        data.iter().fold(addr, |acc, &byte| acc ^ byte)
    }

    pub fn read_edid(&mut self) -> bool {
        info!("Reading EDID...");

        if self.i2c.write(EDID_ADDR, &[0]).is_err() {
            bail!("Failed to write EDID offset")
        }

        Self::delay_us(5000);

        let mut edid = [0u8; 128];
        if self.i2c.read(EDID_ADDR, &mut edid).is_err() {
            bail!("Failed to read EDID")
        }

        if edid[..8] != EDID_HEADER {
            bail!("EDID header invalid: {:02X?}", &edid[..8])
        }

        let manufacturer = ((edid[8] as u16) << 8) | edid[9] as u16;
        info!(
            "Edid: Mfg: {}{}{}, Product: 0x{:04X}, Year: {}",
            (((manufacturer >> 10) & 0x1F) as u8 + b'@') as char,
            (((manufacturer >> 5) & 0x1F) as u8 + b'@') as char,
            ((manufacturer & 0x1F) as u8 + b'@') as char,
            edid[10] as u16 | (edid[11] as u16) << 8,
            1990 + edid[17] as u16
        );
        true
    }

    pub fn test_ddc_read(&mut self) -> bool {
        info!("Testing DDC/CI read (brightness)...");
        let command = [0x51, 0x82, 0x01, 0x10];
        let packet = [
            command[0],
            command[1],
            command[2],
            command[3],
            Self::checksum(DDC_ADDR << 1, &command),
        ];

        if self.i2c.write(DDC_ADDR, &packet).is_err() {
            bail!("DDC write failed")
        }

        Self::delay_us(50000);

        let mut response = [0u8; 12];

        if self.i2c.read(DDC_ADDR, &mut response).is_err() {
            bail!("DDC read failed")
        }

        if response[1] & 0x7F >= 8 && response[2] == 0x02 && response[3] == 0x00 {
            let (max, current) = (
                (response[6] as u16) << 8 | response[7] as u16,
                (response[8] as u16) << 8 | response[9] as u16,
            );
            info!(
                "Brightness: {}/{} ({}%)",
                current,
                max,
                current * 100 / max.max(1)
            );
            true
        } else {
            info!("Unexpected response: {:02X?}", response);
            false
        }
    }
}
