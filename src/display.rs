use core::usize;

use embedded_hal::digital::v2::OutputPin;

pub mod corner;
pub mod words;

const MAX_TIME_ON: u16 = 10_000;
const MIN_TIME_ON: u16 = 0;

type DisplayBuffer = [u16; 10];

pub struct Display {
    pin_latch: crate::hw_config::ShiftregLatch,
    pin_clock: crate::hw_config::ShiftregClock,
    pin_data: crate::hw_config::ShiftregData,
    pin_output_enable: crate::hw_config::ShiftregOutputEnable,

    pub data: DisplayBuffer,

    brightness: u8,
}

impl Display {
    pub fn new(
        pin_latch: crate::hw_config::ShiftregLatch,
        pin_clock: crate::hw_config::ShiftregClock,
        pin_data: crate::hw_config::ShiftregData,
        pin_output_enable: crate::hw_config::ShiftregOutputEnable,
    ) -> Self {
        Self {
            pin_latch,
            pin_clock,
            pin_data,
            pin_output_enable,
            data: [0b0000_0000_0000_0000; 10],
            brightness: 255,
        }
    }

    /// Enables the LED Output
    pub fn enable_output(&mut self) {
        // Set pin to low to enable output
        self.pin_output_enable.set_low().unwrap();
    }

    /// Disables the LED Output
    pub fn disable_output(&mut self) {
        // Set pin to hight to disable output
        self.pin_output_enable.set_high().unwrap();
    }

    /// Shifts a row of data to the shift registers and loads them to the outputs
    fn display_line(&mut self, line: usize) {
        // Set latch to low
        self.pin_latch.set_low().unwrap();
        // Output data for the current line to shifregister
        let mut mask = 0x0001;
        for _ in 0..16 {
            self.pin_clock.set_low().unwrap();
            if (self.data[line] & mask) != 0 {
                self.pin_data.set_high().unwrap();
            } else {
                self.pin_data.set_low().unwrap();
            }
            self.pin_clock.set_high().unwrap();
            mask <<= 1;
        }
        // select the correct line
        for line_bit in 0..16 {
            self.pin_clock.set_low().unwrap();
            if line == line_bit {
                self.pin_data.set_high().unwrap();
            } else {
                self.pin_data.set_low().unwrap();
            }
            self.pin_clock.set_high().unwrap();
        }
        // Store shiftregister in store register
        self.pin_latch.set_high().unwrap();
    }

    /// Update the display with the buffer data
    pub fn update_display(&mut self) {
        // Calculate time for a line to be on
        let time_on: u16 =
            MIN_TIME_ON.saturating_add((MAX_TIME_ON - MIN_TIME_ON) / self.brightness as u16);

        // Output a line
        for line in 0..self.data.len() {
            self.display_line(line);
            arduino_uno::delay_us(time_on);
        }
    }
}
