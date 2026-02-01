#![no_std]
#[allow(unused)]

use log::*;

use esp_hal::clock::CpuClock;
use esp_hal::time::{Duration, Instant};
use esp_hal::peripherals::Peripherals;
use esp_hal::system::Cpu;

pub struct Core {
    peripherals: Peripherals,
}

impl Core {
    pub fn new() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        Core {
            peripherals,
        }
    }

    pub const COUNT: usize = 2;
    pub fn main(self) -> ! {

        info!("Chip:{}", esp_hal::chip!());
        info!("Number of cores: {}", Cpu::COUNT);

        loop {
            info!("Hello world!");
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(500) {}
        }
    }
}