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
    pub led_on_board: LedOnBoard,
    pub ldr_pin: LdrPin,
    pub display: crate::display::Display,
    pub serial: SerialUsb,
    pub btn_birghtness: BtnBrightness,
    pub btn_min: BtnMin,
    pub btn_hour: BtnHour,
}

pub type LdrPin = PC3<Analog>;

pub type Dcf77Pin = PB1<Input<PullUp>>;

pub type LedGreen = PD4<Output>;
pub type LedYellow = PB0<Output>;
pub type LedOnBoard = PB5<Output>;

pub type Rtc = ds1307::Ds1307<I2cMaster<Input<PullUp>>>;
pub type RtcSqwPin = PD2<Input<Floating>>;

pub type ShiftregClock = PB4<Output>;
pub type ShiftregLatch = PB3<Output>;
pub type ShiftregData = PB2<Output>;
pub type ShiftregOutputEnable = PD3<Output>;

pub type BtnBrightness = PD7<Input<Floating>>;
pub type BtnMin = PD6<Input<Floating>>;
pub type BtnHour = PD5<Input<Floating>>;

pub type SerialUsb = Serial<Floating>;
