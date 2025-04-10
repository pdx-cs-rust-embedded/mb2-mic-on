#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use core::arch::asm;

use cortex_m_rt::entry;
extern crate nrf52833_pac;

#[inline(never)]
fn delay(msecs: u32) {
    for _ in 0..8000 * msecs {
        unsafe { asm!("nop") };
    }
}

#[entry]
fn main() -> ! {
    let peripherals = nrf52833_pac::Peripherals::take().unwrap();
    let p0 = peripherals.P0;
    // https://google.github.io/comprehensive-rust/bare-metal/microcontrollers/pacs.html
    p0.pin_cnf[20].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0h1();
        w.sense().disabled();
        w
    });
    p0.outset.write(|w| w.pin20().set());
    delay(3000);
    p0.outclr.write(|w| w.pin20().clear());

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
