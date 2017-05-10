//! User LEDs

use stm32f3x4::{GPIOB, Gpiob, Rcc};

/// All the user LEDs
pub static LEDS: [Led; 4] = [
    Led { i: 6 },
    Led { i: 9 },
    Led { i: 7 },
    Led { i: 8 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOB.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOB.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpiob: &Gpiob, rcc: &Rcc) {
	unsafe {
		// Power up peripherals
		rcc.ahbenr.modify(|_, w| w.iopben().bits(1));

		// Configure pins 7-9 as outputs
		gpiob
		    .moder
		    .modify(
		        |_, w| {
		            w.moder6()
		             .bits(1)
		             .moder8()
		             .bits(1)
		             .moder9()
		             .bits(1)
		             .moder7()
		             .bits(1)
		        },
		    );
	}
}

