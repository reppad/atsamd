# RobotDyn SAMD21 M0-Mini Board Support Crate

This crate provides a type-safe API for working with the [RobotDyn SAMD21 M0-Mini](https://robotdyn.com/samd21-m0-mini-soldered.html).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install the [cargo-hf2 tool](https://crates.io/crates/cargo-hf2) however your
  platform requires

## Uploading an example
Check out [the
repository](https://github.com/atsamd-rs/atsamd/tree/master/boards/samd21_m0_mini/examples)
for examples.

* Be in this directory `cd boards/samd21_m0_mini`
* Put your device in bootloader mode by bridging the `RST` pads _twice_ in
  quick succession. The blue LED (TX) will pulse when the device is in bootloader
  mode.
* Build and upload in one step: `cargo hf2 --release --example blink`
  * Note that if you're using an older `cargo-hf2` that you'll need to specify
    the VID/PID when flashing: `cargo hf2 --vid 0x03eb --pid 0x2402 --release
    --example blink`

Note: This board is factory flashed with an Arduino Zero bootloader (not UF2 compatible), if you haven't installed an UF2 bootloader yourself, you can use bossace tool from Adafruit arduino support package as described in [project readme](https://github.com/atsamd-rs/atsamd#getting-code-onto-the-device-adafruit-m0m4-board-such-as-gemma-m0--feather-m0)
