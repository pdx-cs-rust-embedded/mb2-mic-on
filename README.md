# mb2-mic-on: low…high level Rust pin-setting
Bart Massey 2024

This code turns on the MicroBit v2 microphone, at varying
levels of abstraction:

* `mic-on-raw`: Uses arithmetic and the manual.
* `mic-on-structured`: Hooray, subroutines.
* `mic-on-pac`: Uses `nrf52833-pac`.
* `mic-on-hal`: Uses `nrf52833-hal`.
* `mic-on-board`: Uses `microbit-v2`.
* `mic-on-embassy`: Uses Embassy `microbit-bsp`.

# Acknowledgments

The excellent [Comprehensive
Rust](https://google.github.io/comprehensive-rust/bare-metal/microcontrollers/mmio.html)
contains some very similar code.
