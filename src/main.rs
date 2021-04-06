#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

mod display;
mod hw_config;

#[arduino_uno::entry]
fn main() -> ! {
    let _resources = setup();

    loop {
        unimplemented!()
    }
}

fn setup() -> hw_config::Resources {
    // Take peripherals
    let dp = arduino_uno::Peripherals::take().unwrap();

    // Init IOs
    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Init serial interface
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&pins.ddr),
        57600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "WORDCLOCK Init").ok();

    // Init i2c master
    let i2c = arduino_uno::I2cMaster::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&pins.ddr),
        pins.a5.into_pull_up_input(&pins.ddr),
        50000,
    );

    // LED Pins
    let led_yellow = pins.d8.into_output(&pins.ddr);
    let led_green = pins.d4.into_output(&pins.ddr);

    // Pin with signal from dcf77
    let dcf77_pin = pins.d9.into_floating_input(&pins.ddr);

    // Init Light Depending Resistor
    let adc_settings = arduino_uno::adc::AdcSettings::default();
    let mut adc = arduino_uno::adc::Adc::new(dp.ADC, adc_settings);
    let ldr_pin = pins.a3.into_analog_input(&mut adc);

    // Shift Register
    let shiftreg_clock = pins.d12.into_output(&pins.ddr);
    let shiftreg_latch = pins.d11.into_output(&pins.ddr);
    let shiftreg_data = pins.d10.into_output(&pins.ddr);
    let shiftreg_output_enable = pins.d3.into_output(&pins.ddr);

    // Buttons
    let pin_mode = pins.d7.into_floating_input(&pins.ddr);
    let pin_min = pins.d6.into_floating_input(&pins.ddr);
    let pin_hour = pins.d5.into_floating_input(&pins.ddr);

    // Init rtc - ds1307
    let rtc = ds1307::Ds1307::new(i2c);
    let rtc_sqw_pin = pins.d2.into_floating_input(&pins.ddr);

    // Return the resources
    hw_config::Resources {
        rtc,
        rtc_sqw_pin,
        led_green,
        led_yellow,
        dcf77_pin,
        ldr_pin,
        shiftreg_clock,
        shiftreg_latch,
        shiftreg_data,
        shiftreg_output_enable,
        pin_mode,
        pin_min,
        pin_hour,
        serial,
    }
}
