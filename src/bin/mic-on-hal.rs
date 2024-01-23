#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio,
    pac::Peripherals,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let p0 = peripherals.P0;
    // https://google.github.io/comprehensive-rust/bare-metal/microcontrollers/pacs.html
    let p0 = gpio::p0::Parts::new(p0);

    // XXX No high-drive.
    let _p0_20 = p0.p0_20.into_push_pull_output(gpio::Level::High);

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
