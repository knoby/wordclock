use arduino_uno::{
    hal::port::{
        mode::{Analog, Floating, Input, Output, PullUp},
        portb::*,
        portc::*,
        portd::*,
    },
    I2cMaster, Serial,
};

pub struct Resources {
    pub rtc: Rtc,
    pub rtc_sqw_pin: RtcSqwPin,
    pub led_yellow: LedYellow,
    pub led_green: LedGreen,
    pub dcf77_pin: Dcf77Pin,
    pub ldr_pin: LdrPin,
    pub shiftreg_clock: ShiftregClock,
    pub shiftreg_data: ShiftregData,
    pub shiftreg_latch: ShiftregLatch,
    pub shiftreg_output_enable: ShiftregOutputEnable,
    pub pin_mode: PinMode,
    pub pin_min: PinMin,
    pub pin_hour: PinHour,
    pub serial: SerialUsb,
}

pub type LdrPin = PC3<Analog>;

pub type Dcf77Pin = PB1<Input<Floating>>;

pub type LedGreen = PD4<Output>;
pub type LedYellow = PB0<Output>;

pub type Rtc = ds1307::Ds1307<I2cMaster<Input<PullUp>>>;
pub type RtcSqwPin = PD2<Input<Floating>>;

pub type ShiftregClock = PB4<Output>;
pub type ShiftregLatch = PB3<Output>;
pub type ShiftregData = PB2<Output>;
pub type ShiftregOutputEnable = PD3<Output>;

pub type PinMode = PD7<Input<Floating>>;
pub type PinMin = PD6<Input<Floating>>;
pub type PinHour = PD5<Input<Floating>>;

pub type SerialUsb = Serial<Floating>;
