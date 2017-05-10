//! User LEDs

use stm32l4x6::{GPIOA, Gpioa, Rcc};

/// All the user LEDs
pub static LEDS: [Led; 1] = [
    Led { i: 5 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOA.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOA.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpioa: &Gpioa, rcc: &Rcc) {
	unsafe {
		// Power up peripherals
		rcc.ahb2enr.modify(|_, w| w.gpioaen().bits(1));

		// Configure pins 7-9 as outputs
		gpioa
		    .moder
		    .modify(
		        |_, w| {
		            w.moder5()
		             .bits(1)
		        },
		    );
	}
}

