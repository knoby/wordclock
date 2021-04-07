#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use core::{
    cell::{Cell, RefCell},
    ops::Shl,
};

use arduino_uno::prelude::*;
use avr_device::interrupt::Mutex;
use ds1307::Rtcc;

mod display;
mod hw_config;

// Resources init in main and used in interrupt
static mut DCF77_PIN: Option<hw_config::Dcf77Pin> = None;
static mut LED_GREEN: Option<hw_config::LedGreen> = None;
static mut LED_YELLOW: Option<hw_config::LedYellow> = None;

// Resources used in main and in interrupt
static DCF77_PIN_STATES: Mutex<RefCell<[Option<bool>; 8]>> = Mutex::new(RefCell::new([None; 8]));

static MILLIS_COUNTER: avr_device::interrupt::Mutex<Cell<u32>> =
    avr_device::interrupt::Mutex::new(Cell::new(0));

#[arduino_uno::entry]
fn main() -> ! {
    let mut resources = setup();
    unsafe { avr_device::interrupt::enable() };

    // Clear Display
    resources.display.data = [0xfffe; 10];
    resources.display.enable_output();

    // Init Clock
    resources.rtc.enable_square_wave_output().unwrap();
    if 2021 == resources.rtc.get_year().unwrap() {
        resources.led_on_board.set_high().void_unwrap();
    }

    // Init DCF77 Decoder
    let mut dcf77 = dcf77::SimpleDCF77Decoder::new();

    loop {
        // DCF77 Decoding
        {
            let states = avr_device::interrupt::free(|cs| DCF77_PIN_STATES.borrow(cs).take());
            states
                .iter()
                .filter(|state| state.is_some())
                .map(|state| state.unwrap())
                .for_each(|state| dcf77.read_bit(state));
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    unsafe { LED_YELLOW.as_mut().unwrap().toggle().void_unwrap() };
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    static mut COUNTER: u8 = 0;

    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + 1);
    });

    if unsafe { COUNTER == 0 } {
        // Read Pin state

        // Safe, bacause we only access this in this interrupt
        let dcf77_pin_state = unsafe { DCF77_PIN.as_mut().unwrap().is_high() }.void_unwrap();
        unsafe {
            if dcf77_pin_state {
                LED_GREEN.as_mut().unwrap().set_high().void_unwrap();
            } else {
                LED_GREEN.as_mut().unwrap().set_low().void_unwrap();
            }
        }
        // Get access to the list of states and add to the list
        avr_device::interrupt::free(|cs| {
            let mut list = DCF77_PIN_STATES.borrow(cs).borrow_mut();
            if let Some(item) = (*list).iter_mut().find(|item| item.is_none()) {
                *item = Some(dcf77_pin_state);
            }
        });
    }

    unsafe {
        COUNTER += 1;
        if COUNTER >= 10 {
            COUNTER = 0;
        }
    }
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
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
    unsafe { LED_GREEN = Some(pins.d4.into_output(&pins.ddr)) };
    unsafe { LED_YELLOW = Some(pins.d8.into_output(&pins.ddr)) };
    let led_on_board = pins.d13.into_output(&pins.ddr);

    // Pin with signal from dcf77
    unsafe {
        DCF77_PIN = Some(pins.d9.into_pull_up_input(&pins.ddr));
    }

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
    let pin_mode = pins.d7.into_floating_input(&pins.ddr);
    let pin_min = pins.d6.into_floating_input(&pins.ddr);
    let pin_hour = pins.d5.into_floating_input(&pins.ddr);

    // Init rtc - ds1307
    let rtc = ds1307::Ds1307::new(i2c);
    let rtc_sqw_pin = pins.d2.into_floating_input(&pins.ddr);
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
        pin_mode,
        display,
        pin_min,
        pin_hour,
        serial,
    }
}
