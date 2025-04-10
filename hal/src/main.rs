#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{gpio, pac::Peripherals, timer};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let p0 = peripherals.P0;
    // https://google.github.io/comprehensive-rust/bare-metal/microcontrollers/pacs.html
    let p0 = gpio::p0::Parts::new(p0);

    let timer = peripherals.TIMER0;
    let mut timer = timer::Timer::new(timer);

    // XXX No high-drive.
    let mut p0_20 = p0.p0_20.into_push_pull_output(gpio::Level::High);
    timer.delay_ms(3000u32);
    p0_20.set_low().unwrap();

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
