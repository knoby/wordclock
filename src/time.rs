use ds1307::Rtcc;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Time {
    sec: u8,
    min: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u16,
}

impl Time {
    pub fn try_from_rtc(rtc: &mut crate::hw_config::Rtc) -> Result<Self, ()> {
        let year = rtc.get_year().map_err(|_| ())?;
        let month = rtc.get_month().map_err(|_| ())?;
        let day = rtc.get_day().map_err(|_| ())?;
        let hour = match rtc.get_hours().map_err(|_| ())? {
            ds1307::Hours::H24(h) => h,
            ds1307::Hours::AM(h) => h,
            ds1307::Hours::PM(h) => h + 12,
        };
        let min = rtc.get_minutes().map_err(|_| ())?;
        let sec = rtc.get_seconds().map_err(|_| ())?;
        Ok(Self {
            min,
            sec,
            hour,
            day,
            month,
            year,
        })
    }

    pub fn set_rtc(&self, rtc: &mut crate::hw_config::Rtc) -> Result<(), ()> {
        rtc.set_year(self.year).map_err(|_| ())?;
        rtc.set_month(self.month).map_err(|_| ())?;
        rtc.set_day(self.day).map_err(|_| ())?;
        rtc.set_hours(ds1307::Hours::H24(self.hour))
            .map_err(|_| ())?;
        rtc.set_minutes(self.min).map_err(|_| ())?;
        rtc.set_seconds(self.sec).map_err(|_| ())?;

        Ok(())
    }

    pub fn try_from_dcf77(dcf77: &dcf77::SimpleDCF77Decoder) -> Result<Self, ()> {
        // Get DCF77 time object
        let dcf77_time = dcf77::DCF77Time::new(dcf77.raw_data());

        dcf77_time.validate_start()?;

        let year = dcf77_time.year_unchecked();
        let month = dcf77_time.month_unchecked();
        let day = dcf77_time.day().map_err(|_| ())?;
        let hour = dcf77_time.hours().map_err(|_| ())?;
        let min = dcf77_time.minutes().map_err(|_| ())?;
        let sec = 0;

        Ok(Self {
            min,
            sec,
            hour,
            day,
            month,
            year,
        })
    }

    pub fn minutes(&self) -> u8 {
        self.min
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn seconds(&self) -> u8 {
        self.sec
    }

    pub fn inc_minutes(&self) -> Self {
        let mut time = *self;
        time.min += 1;
        if time.min >= 60 {
            time.hour += 1;
            time.min = 0;
        }
        if time.hour >= 24 {
            time.hour = 0;
        }

        time
    }

    pub fn inc_hours(&self) -> Self {
        let mut time = *self;
        time.hour += 1;
        if time.hour >= 24 {
            time.hour = 0;
        }
        time
    }
}
