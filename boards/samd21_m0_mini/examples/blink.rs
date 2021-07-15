#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
extern crate samd21_m0_mini as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led1 = pins.led_rx.into_open_drain_output(&mut pins.port);
    let mut led2 = pins.led_tx.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut counter = 0u8;
    loop {
        counter = counter.wrapping_add(1);
        delay.delay_ms(100u8);
        if counter & (1 << 1) != 0 {
            led1.toggle();
        }
        if counter & (1 << 2) != 0 {
            led2.toggle();
        }
    }
}
