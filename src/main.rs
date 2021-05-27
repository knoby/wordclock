#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use core::cell::{Cell, RefCell};

use arduino_uno::prelude::*;
use avr_device::interrupt::{free, Mutex};
use util::{SharedInput, SharedOutput};

mod display;
mod hw_config;
mod time;
mod util;

// Resources init in main and used in interrupt
static DCF77_PIN: Mutex<RefCell<Option<hw_config::Dcf77Pin>>> = Mutex::new(RefCell::new(None));
static LED_GREEN: Mutex<RefCell<Option<hw_config::LedGreen>>> = Mutex::new(RefCell::new(None));
static LED_YELLOW: Mutex<RefCell<Option<hw_config::LedYellow>>> = Mutex::new(RefCell::new(None));

// Resources used in main and in interrupt
static DCF77_PIN_STATES: Mutex<RefCell<[Option<bool>; 10]>> = Mutex::new(RefCell::new([None; 10])); // Can hold samples up to 1s ... schould be enough

// Counter for rising edges of the SQW signal
static SECOND_COUNTER: Mutex<Cell<u8>> = Mutex::new(Cell::new(50));

// Millis like counter
static MILLIS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[arduino_uno::entry]
fn main() -> ! {
    let mut resources = setup();
    unsafe { avr_device::interrupt::enable() };

    let mut btn_birghtness_old = resources.btn_birghtness.is_high().void_unwrap();
    let mut btn_min_old = resources.btn_min.is_high().void_unwrap();
    let mut btn_hour_old = resources.btn_hour.is_high().void_unwrap();

    // Clear Display
    resources.display.clear();
    resources.display.enable_output();

    // Init DCF77 Decoder
    let mut dcf77 = dcf77::SimpleDCF77Decoder::new();

    // Create time or use a spare value
    let mut dcf77_time = time::Time::default();
    let mut current_time = time::Time::try_from_rtc(&mut resources.rtc).unwrap_or_default();
    resources.display.update_data(&current_time);

    loop {
        // Update dcf77 Decoder Struct
        let states = free(|cs| DCF77_PIN_STATES.borrow(cs).take());
        for state in states
            .iter()
            .filter(|state| state.is_some())
            .map(|state| state.unwrap())
        {
            // eval Bit
            dcf77.read_bit(state);
            // Check if last bit recived
            if dcf77.end_of_cycle() {
                // Decode the time information
                if let Ok(new_dcf77_time) = time::Time::try_from_dcf77(&dcf77) {
                    let test_time = dcf77_time.inc_minutes();
                    if test_time == new_dcf77_time {
                        // two times in a row valid signal was found
                        new_dcf77_time.set_rtc(&mut resources.rtc).ok();
                    }
                    // Save reading for next cycle
                    dcf77_time = new_dcf77_time;
                    resources.led_on_board.set_high().void_unwrap();
                } else {
                    resources.led_on_board.set_low().void_unwrap();
                }
            }
        }

        // Check if update of the time from rtc is needed
        let update_needed = free(|cs| SECOND_COUNTER.borrow(cs).get()) >= 60;
        if update_needed {
            // Read from rtc

            if let Ok(time) = time::Time::try_from_rtc(&mut resources.rtc) {
                current_time = time;
            } else {
                current_time.inc_minutes();
            }
            // Reset the seconds counter
            free(|cs| SECOND_COUNTER.borrow(cs).set(current_time.seconds()));
            // Update the display with the current time
            resources.display.update_data(&current_time);
        }

        // Check if the brightness btn was pressed
        let btn_state = resources.btn_birghtness.is_high().void_unwrap();
        if btn_state && !btn_birghtness_old {
            let current_brightness = resources.display.brightness();
            match current_brightness {
                0 => {
                    resources.display.enabled();
                    resources.display.set_brightness(51);
                }
                255 => {
                    resources.display.disable_output();
                    resources.display.set_brightness(0);
                }
                _ => {
                    resources
                        .display
                        .set_brightness(current_brightness.saturating_add(51));
                }
            }
        };
        btn_birghtness_old = btn_state;

        // Check min / hour btn
        on_rising_edge(&resources.btn_hour, &mut btn_hour_old, &|| {
            current_time.inc_hours();
        });
        on_rising_edge(&resources.btn_min, &mut btn_min_old, &|| {
            current_time.inc_minutes();
        });

        // Update the Display
        resources.display.update_display();
    }
}

/// Every Rising Edge of the square wave signal of the rtc die is counted
/// With this counter we can determin when it is time to update the display (every 60s)
#[avr_device::interrupt(atmega328p)]
fn INT0() {
    free(|cs| {
        LED_YELLOW.toggle(cs).void_unwrap();
        SECOND_COUNTER.borrow(cs).update(|seconds| seconds + 1);
    });
}

/// Cyclic Function that is called every milli second
/// every 10th call the state of the dcf77 reciver is polled and queued to the main task
/// Every Cycle a variable is increased to support a millis function like in arduino
#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    static mut COUNTER: u8 = 0;

    // Inc Millis counter
    free(|cs| {
        MILLIS_COUNTER
            .borrow(cs)
            .update(|millis| millis.wrapping_add(1));
    });

    match unsafe { COUNTER } {
        0 => {
            free(|cs| {
                let dcf77_pin_state = DCF77_PIN.is_high(cs).void_unwrap();

                LED_GREEN.set(cs, dcf77_pin_state).void_unwrap();

                // Get access to the list of states and add to the list
                let mut list = DCF77_PIN_STATES.borrow(cs).borrow_mut();
                if let Some(item) = (*list).iter_mut().find(|item| item.is_none()) {
                    *item = Some(dcf77_pin_state);
                }
            });
        }
        1 => (),
        2 => (),
        3 => (),
        4 => (),
        5 => (),
        6 => (),
        7 => (),
        8 => (),
        9 => (),
        _ => (),
    };

    unsafe {
        COUNTER += 1;
        if COUNTER >= 10 {
            COUNTER = 0;
        }
    }
}

fn on_rising_edge(
    pin: &dyn embedded_hal::digital::v2::InputPin<Error = void::Void>,
    old_state: &mut bool,
    f: &dyn Fn(),
) {
    let state = pin.is_high().unwrap();
    if state && !*old_state {
        f();
    }
    *old_state = state;
}

#[allow(dead_code)]
fn millis() -> u32 {
    free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

fn setup() -> hw_config::Resources {
    // Take peripherals
    let dp = arduino_uno::Peripherals::take().unwrap();

    // Init IOs
    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Init serial interface
    let serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&pins.ddr),
        57600.into_baudrate(),
    );

    // Init i2c master
    let i2c = arduino_uno::I2cMaster::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&pins.ddr),
        pins.a5.into_pull_up_input(&pins.ddr),
        50000,
    );

    // LED Pins
    let led_green = Some(pins.d4.into_output(&pins.ddr));
    free(|cs| LED_GREEN.borrow(cs).replace(led_green));
    let led_yellow = Some(pins.d8.into_output(&pins.ddr));
    free(|cs| LED_YELLOW.borrow(cs).replace(led_yellow));
    let led_on_board = pins.d13.into_output(&pins.ddr);

    // Pin with signal from dcf77
    let dcf77_pin = Some(pins.d9.into_pull_up_input(&pins.ddr));
    free(|cs| DCF77_PIN.borrow(cs).replace(dcf77_pin));

    // Init Light Depending Resistor
    let adc_settings = arduino_uno::adc::AdcSettings::default();
    let mut adc = arduino_uno::adc::Adc::new(dp.ADC, adc_settings);
    let ldr_pin = pins.a3.into_analog_input(&mut adc);

    // Shift Register
    let shiftreg_clock = pins.d12.into_output(&pins.ddr);
    let shiftreg_latch = pins.d11.into_output(&pins.ddr);
    let shiftreg_data = pins.d10.into_output(&pins.ddr);
    let shiftreg_output_enable = pins.d3.into_output(&pins.ddr);

    let display = display::Display::new(
        shiftreg_latch,
        shiftreg_clock,
        shiftreg_data,
        shiftreg_output_enable,
    );

    // Buttons
    let btn_birghtness = pins.d7.into_floating_input(&pins.ddr);
    let btn_min = pins.d6.into_floating_input(&pins.ddr);
    let btn_hour = pins.d5.into_floating_input(&pins.ddr);

    // Init rtc - ds1307
    let mut rtc = ds1307::Ds1307::new(i2c);
    let rtc_sqw_pin = pins.d2.into_floating_input(&pins.ddr);
    // Enable SQW Output
    rtc.enable_square_wave_output().unwrap();
    // Enable interrupt on sqw pin
    dp.EXINT.eicra.write(|w| w.isc0().val_0x03()); // Rising Edge on INT0
    dp.EXINT.eimsk.write(|w| w.int0().set_bit()); // Enable the Interrupt

    // Create Timer with 1ms overflow for cyclic tasks
    dp.TC0.tccr0a.write(|w| w.wgm0().ctc());
    dp.TC0.ocr0a.write(|w| unsafe { w.bits(250) });
    dp.TC0.tccr0b.write(|w| w.cs0().prescale_64());
    dp.TC0.timsk0.write(|w| w.ocie0a().set_bit());

    // Return the resources
    hw_config::Resources {
        rtc,
        rtc_sqw_pin,
        led_on_board,
        ldr_pin,
        display,
        serial,
        btn_birghtness,
        btn_min,
        btn_hour,
    }
}
