#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::{gpio, timer}};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = timer::Timer::new(board.TIMER0);
    // XXX No high-drive.
    let mut mic_run = board
        .microphone_pins
        .mic_run
        .into_push_pull_output(gpio::Level::High);
    timer.delay_ms(3000u32);
    mic_run.set_low().unwrap();

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
