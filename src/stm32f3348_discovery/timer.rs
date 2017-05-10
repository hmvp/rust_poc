//! Periodic timer

use core::u16;

use cast::{u16, u32};
use stm32f3x4::{Rcc, Tim7};

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

// pub const AHB: u32 = 8_000_000;
pub const APB1: u32 = 8_000;
pub const APB2: u32 = 8_000;

/// Periodic timer
///
/// # Interrupts
///
/// - `Tim7Irq` - update event
#[derive(Clone, Copy)]
pub struct Timer<'a>(pub &'a Tim7);

impl<'a> Timer<'a> {
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE After initialization, the timer will be in the paused state.
    pub fn init(&self, rcc: &Rcc, frequency: u32) {
        let tim7 = self.0;

		unsafe {
		    // Power up peripherals
		    rcc.apb1enr.modify(|_, w| w.tim7en().bits(1));

		    let ratio = APB1 / frequency;
		    let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
		    tim7.psc.write(|w| w.psc().bits(psc));
		    let arr = u16(ratio / u32(psc + 1)).unwrap();
		    tim7.arr.write(|w| w.arr().bits(arr));

		    tim7.dier.write(|w| w.uie().bits(1));
		    tim7.cr1.write(|w| w.opm().bits(0));
		}
    }

    /// Clears the update event flag
    ///
    /// Returns `Err` if no update event has occurred
    pub fn clear_update_flag(&self) -> Result<()> {
        let tim7 = self.0;

        if tim7.sr.read().uif().bits() == 0 {
            Err(Error { _0: () })
        } else {
			unsafe {
            	self.0.sr.modify(|_, w| w.uif().bits(0));
			}
            Ok(())
        }
    }

    /// Resumes the timer count
    pub fn resume(&self) {
		unsafe {
    	    self.0.cr1.modify(|_, w| w.cen().bits(1));
		}
    }

    /// Pauses the timer
    pub fn pause(&self) {
		unsafe {
	        self.0.cr1.modify(|_, w| w.cen().bits(0));
		}
    }
}
