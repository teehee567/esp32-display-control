#![no_std]
#[allow(unused)]
use log::*;

use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::peripherals::Peripherals;
use esp_hal::system::Cpu;
use esp_hal::time::{Duration, Instant};

mod vcp;
use vcp::VcpController;

pub struct Core {
    peripherals: Peripherals,
}

// https://docs.espressif.com/projects/rust/

impl Core {
    pub fn new() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        Core { peripherals }
    }

    pub const COUNT: usize = 2;
    pub fn main(self) -> ! {
        info!("Chip:{}", esp_hal::chip!());
        info!("Number of cores: {}", Cpu::COUNT);

        let i2c = I2c::new(self.peripherals.I2C0, Config::default())
            .unwrap()
            .with_sda(self.peripherals.GPIO1)
            .with_scl(self.peripherals.GPIO2);

        let mut vcp = VcpController::new(i2c);
        vcp.read_edid();
        vcp.test_ddc_read();

        loop {
            info!("Hello world!");
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(2000) {}
        }
    }
}
