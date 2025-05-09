# mb2-mic-on: low…high level Rust pin-setting
Bart Massey 2024

This code turns on the MicroBit v2 microphone (and thus its
"mic on" LED) at varying levels of
abstraction:

* `raw`: Uses arithmetic and the manual.
* `structured`: Hooray, subroutines.
* `pac`: Uses `nrf52833-pac`.
* `hal`: Uses `nrf52833-hal`.
* `board`: Uses `microbit-v2`.

All but `raw` turn the mic back off after about three
seconds.

You will need to build the pre-`hal` code with `--release`
to get the timing right. The delay spin-loop will generate a
lot of extra instructions per iteration in debug mode, since
it will call an iterator `next()` method each iteration.

These [setup](./SETUP.md) commands will get your Linux box
set up to run this stuff. Similar things for other platforms.

# Acknowledgments

The excellent [Comprehensive
Rust](https://google.github.io/comprehensive-rust/bare-metal/microcontrollers/mmio.html)
used to contain some very similar code.
