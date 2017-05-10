//! User LEDs

use stm32l4x6::{GPIOC, Gpioc, Rcc};

/// All the user BUTTONS
pub static BUTTONS: [Button; 1] = [
    Button(),
];

/// An Button
pub struct Button();

impl Button {
    /// Turns on the LED
    pub fn state(&self) -> bool {
        // NOTE(safe) atomic write
		unsafe {
        	(*GPIOC.get()).idr.read().idr13().bits() == 1
		}
    }
}

/// Initializes all the user LEDs
pub fn init(gpioc: &Gpioc, rcc: &Rcc) {
	unsafe {
		// Power up peripherals
		rcc.ahb2enr.modify(|_, w| w.gpiocen().bits(1));

		// Configure pins 0 as inputs
		gpioc
		    .moder
		    .modify(
		        |_, w| {
		            w.moder13()
		             .bits(0)
		        },
		    );
	}
}

