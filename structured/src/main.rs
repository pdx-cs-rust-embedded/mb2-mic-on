#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use cortex_m_rt::entry;

const GPIO_BASE: usize = 0x50000000;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Port {
    P0 = 0,
    P1 = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Dir {
    Input = 0,
    Output = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Input {
    Connect = 0,
    Disconnect = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Pull {
    Disabled = 0,
    Pulldown = 1,
    Pullup = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Drive {
    S0S1 = 0,
    H0S1 = 1,
    S0H1 = 2,
    H0H1 = 3,
    D0S1 = 4,
    D0H1 = 5,
    S0D1 = 6,
    H0D1 = 7,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Sense {
    Disabled = 0,
    // XXX 1?
    High = 2,
    Low = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Output {
    Low = 0,
    High = 1,
}

fn port_base(port: Port) -> usize {
    GPIO_BASE + port as usize * 0x300
}

fn write_pin_config(
    port: Port,
    pin: usize,
    dir: Dir,
    input: Input,
    pull: Pull,
    drive: Drive,
    sense: Sense,
) {
    let addr = (port_base(port) + 0x700 + pin * 4) as *mut u32;
    let mut config = 0;
    #[allow(clippy::identity_op)]
    {
        config |= (dir as u32) << 0;
    }
    config |= (input as u32) << 1;
    config |= (pull as u32) << 2;
    config |= (drive as u32) << 8;
    config |= (sense as u32) << 16;
    unsafe {
        addr.write_volatile(config);
    }
}

fn set_pin(port: Port, pin: usize, output: Output) {
    match output {
        Output::Low => {
            let addr_clr = (port_base(port) + 0x50c) as *mut u32;
            unsafe {
                addr_clr.write_volatile(1 << pin);
            }
        }
        Output::High => {
            let addr_set = (port_base(port) + 0x508) as *mut u32;
            unsafe {
                addr_set.write_volatile(1 << pin);
            }
        }
    }
}

#[entry]
fn main() -> ! {
    // Write 1 to MIC_POWER pin.
    write_pin_config(
        Port::P0,
        20,
        Dir::Output,
        Input::Disconnect,
        Pull::Disabled,
        Drive::S0H1,
        Sense::Disabled,
    );

    set_pin(Port::P0, 20, Output::High);

    // Loop forever.
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
