#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use cortex_m_rt::entry;
use microbit::{board::Board, hal::gpio};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    // XXX No high-drive.
    let _mic_run = board
        .microphone_pins
        .mic_run
        .into_push_pull_output(gpio::Level::High);

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
