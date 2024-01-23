#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_pac;

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

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
