use core::usize;

use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

pub mod corner;
pub mod words;

const MAX_TIME_ON: u16 = 20_000;
const MIN_TIME_ON: u16 = 1_000;

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

    /// Query the current Ouptut status
    pub fn enabled(&self) -> bool {
        self.pin_output_enable.is_set_low().unwrap()
    }

    /// Query the current brightness
    pub fn brightness(&self) -> u8 {
        self.brightness
    }

    /// Sets the brightness
    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
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
            if self.data[line] != 0xffff {
                // Skip Blank lines
                self.display_line(line);
                arduino_uno::delay_us(time_on);
            }
        }
    }

    /// Clear the display
    pub fn clear(&mut self) {
        self.data = [0xffff; 10];
    }

    /// Update the data with the information from a datetime
    pub fn update_data(&mut self, time: &crate::time::Time) {
        self.clear();
        // Set Obvious data
        self.set_line(words::ESIST);
        // Set Corner
        self.set_sub_minutes(time.minutes());
        // Set Hour Display text
        self.set_hours(time.hour(), time.minutes());
        // Set Minutes Dispaly text
        self.set_minutes(time.minutes());
    }

    fn set_hours(&mut self, hour: u8, min: u8) {
        // Calculate the houre value that schould be displayed
        let mut hours_display = hour;
        if min >= 30 {
            hours_display = hour + 1;
            if hours_display >= 12 {
                hours_display = 1;
            }
        }
        match hours_display {
            0 => self.set_line(words::ZWOELF_HOUR),
            1 => self.set_line(words::EINS_HOUR),
            2 => self.set_line(words::ZWEI_HOUR),
            3 => self.set_line(words::DREI_HOUR),
            4 => self.set_line(words::VIER_HOUR),
            5 => self.set_line(words::FUENF_HOUR),
            6 => self.set_line(words::SECHS_HOUR),
            7 => self.set_line(words::SIEBEN_HOUR),
            8 => self.set_line(words::ACHT_HOUR),
            9 => self.set_line(words::NEUN_HOUR),
            10 => self.set_line(words::ZEHN_HOUR),
            11 => self.set_line(words::ELF_HOUR),
            12 => self.set_line(words::ZWOELF_HOUR),
            _ => unreachable!(),
        }
    }

    fn set_minutes(&mut self, min: u8) {
        let minutes_round = min - (min % 5);
        match minutes_round {
            0 => self.set_line(words::UHR),
            5 => {
                self.set_line(words::FUENF);
                self.set_line(words::NACH);
            }
            10 => {
                self.set_line(words::ZEHN);
                self.set_line(words::NACH);
            }
            15 => {
                self.set_line(words::VIERTEL);
                self.set_line(words::NACH);
            }
            20 => {
                self.set_line(words::ZWANZIG);
                self.set_line(words::NACH);
            }
            25 => {
                self.set_line(words::FUENF);
                self.set_line(words::VOR);
                self.set_line(words::HALB);
            }
            30 => {
                self.set_line(words::HALB);
            }
            35 => {
                self.set_line(words::FUENF);
                self.set_line(words::NACH);
                self.set_line(words::HALB);
            }
            40 => {
                self.set_line(words::ZWANZIG);
                self.set_line(words::VOR);
            }
            45 => {
                self.set_line(words::VIERTEL);
                self.set_line(words::VOR);
            }
            50 => {
                self.set_line(words::ZEHN);
                self.set_line(words::VOR);
            }
            55 => {
                self.set_line(words::FUENF);
                self.set_line(words::VOR);
            }
            _ => unreachable!(),
        }
    }

    fn set_sub_minutes(&mut self, min: u8) {
        // Set Corner
        let corner_count = min % 5;
        if corner_count >= 1 {
            self.set_line(corner::TOP_LEFT);
        }
        if corner_count >= 2 {
            self.set_line(corner::TOP_RIGHT);
        }
        if corner_count >= 3 {
            self.set_line(corner::BOTTOM_LEFT);
        }
        if corner_count >= 4 {
            self.set_line(corner::BOTTOM_RIGHT);
        }
    }

    pub fn set_line(&mut self, line: (usize, u16)) {
        self.data[line.0] ^= line.1;
    }
}
