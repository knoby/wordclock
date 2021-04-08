use core::{borrow::Borrow, cell::RefCell};

use avr_device::interrupt::Mutex;

pub trait SetOutput<T>
where
    T: embedded_hal::digital::v2::OutputPin,
{
    fn set(&mut self, state: bool) -> Result<(), T::Error>;
}

impl<T> SetOutput<T> for T
where
    T: embedded_hal::digital::v2::OutputPin,
{
    fn set(&mut self, state: bool) -> Result<(), T::Error> {
        if state {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub trait SharedInput<T>
where
    T: embedded_hal::digital::v2::InputPin,
{
    fn is_high(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<bool, T::Error>;
    fn is_low(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<bool, T::Error>;
}

impl<T> SharedInput<T> for Mutex<RefCell<Option<T>>>
where
    T: embedded_hal::digital::v2::InputPin,
{
    fn is_high(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<bool, T::Error> {
        self.borrow(cs).borrow().as_ref().unwrap().is_high()
    }

    fn is_low(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<bool, T::Error> {
        self.borrow(cs).borrow().as_ref().unwrap().is_low()
    }
}

pub trait SharedOutput<T>
where
    T: embedded_hal::digital::v2::StatefulOutputPin,
{
    fn set_high(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error>;
    fn set_low(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error>;
    fn toggle(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error>;
    fn set(&self, cs: &avr_device::interrupt::CriticalSection, state: bool)
        -> Result<(), T::Error>;
}

impl<T> SharedOutput<T> for Mutex<RefCell<Option<T>>>
where
    T: embedded_hal::digital::v2::StatefulOutputPin,
{
    fn set_high(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error> {
        self.borrow(cs).borrow_mut().as_mut().unwrap().set_high()
    }

    fn set_low(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error> {
        self.borrow(cs).borrow_mut().as_mut().unwrap().set_low()
    }
    fn toggle(&self, cs: &avr_device::interrupt::CriticalSection) -> Result<(), T::Error> {
        let mut pin = self.borrow(cs).borrow_mut();
        let pin = pin.as_mut().unwrap();
        if pin.is_set_high()? {
            pin.set_low()
        } else {
            pin.set_high()
        }
    }

    fn set(
        &self,
        cs: &avr_device::interrupt::CriticalSection,
        state: bool,
    ) -> Result<(), T::Error> {
        if state {
            self.set_high(cs)
        } else {
            self.set_low(cs)
        }
    }
}
