// from xiao_m0

#![no_std]

pub extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::common::*;

pub use hal::target_device as pac;

use hal::prelude::*;
use hal::{
    clock::GenericClockController,
    define_pins,
    gpio::PfD,
    gpio::{Floating, Input, Port},
    pad::PadPin,
    sercom::{I2CMaster2, SPIMaster0, UART4},
    target_device,
    time::Hertz,
};

#[cfg(feature = "usb")]
use hal::gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    struct Pins,
    target_device: target_device,

    /// Pin A0/D0/DAC
    pin a0 = a2,
    /// Pin A1/D1
    pin a1 = b8,
    /// Pin A2/D2
    pin a2 = b9,
    /// Pin A3/D3
    pin a3 = a4,
    /// Pin A4/D4/SDA
    pin a4 = a5,
    /// Pin A5/D5/SCL
    pin a5 = b2,
    pin rx = a11,
    pin tx = a10,
    //pin d0 = a11,
    //pin d1 = a10,
    //pin d2 = a14,
    //pin d3 = a9,
    pin d4 = a8,
    pin d5 = a15,
    pin d6 = a20,
    pin d7 = a21,
    pin d8 = a6,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,
    pin d13 = a17,

    /// On-board blue 'RX' LED.
    pin led_rx = b3,
    /// On-board blue 'TX' LED.
    pin led_tx = a27,

    /// The USB D- pad.
    pin usb_dm = a24,
    /// The USB D+ pad.
    pin usb_dp = a25,
);

/// Convenience function for setting up the TX (A6/D6) and RX (A7/D7) pins as a
/// UART operating at `baud`.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    a7: gpio::Pb9<Input<Floating>>,
    a6: gpio::Pb8<Input<Floating>>,
    port: &mut Port,
) -> UART4<hal::sercom::Sercom4Pad1<gpio::Pb9<PfD>>, hal::sercom::Sercom4Pad0<gpio::Pb8<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        baud.into(),
        sercom4,
        pm,
        (a7.into_pad(port), a6.into_pad(port)),
    )
}

/// Convenience function for setting up the A4/D4/SDA and A5/D5/SCL pins as an
/// I2C master operating at `speed`.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    speed: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    a4: gpio::Pa8<Input<Floating>>,
    a5: gpio::Pa9<Input<Floating>>,
    port: &mut Port,
) -> hal::sercom::I2CMaster2<
    hal::sercom::Sercom2Pad0<gpio::Pa8<gpio::PfD>>,
    hal::sercom::Sercom2Pad1<gpio::Pa9<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        speed.into(),
        sercom2,
        pm,
        a4.into_pad(port),
        a5.into_pad(port),
    )
}

/// Convenience function for setting up the A8/D8/SCK, A10/D10/MOSI, and
/// A9/D9/MISO pins as an SPI master in SPI mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    speed: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sck: gpio::Pa7<Input<Floating>>,
    mosi: gpio::Pa6<Input<Floating>>,
    miso: gpio::Pa5<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster0<
    hal::sercom::Sercom0Pad1<gpio::Pa5<gpio::PfD>>,
    hal::sercom::Sercom0Pad2<gpio::Pa6<gpio::PfD>>,
    hal::sercom::Sercom0Pad3<gpio::Pa7<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom0,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
