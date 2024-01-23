#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Write 1 to MIC_POWER pin.
    unsafe {
        // GPIO P0 base address.
        let base = 0x50000000_usize;
        // GPIO P0 configuration register.
        let cr = (base + 0x700 + 4 * 20) as *mut u32;
        let mut config = 0;
        // A: Output pin.
        config |= 1 << 0;
        // B: Disable input buffer.
        config |= 1 << 1;
        // C: Disable pull-up and pull-down resistors.
        config |= 0 << 2;
        // D: Standard-drive 0, high-drive 1.
        config |= 2 << 8;
        // E: Pin sense.
        config |= 0 << 16;
        cr.write_volatile(config);
        // GPIO P0 ouput bit-setting register.
        let out_set = (base + 0x508) as *mut u32;
        // Set P0.19.
        out_set.write_volatile(1 << 20);
    }

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
