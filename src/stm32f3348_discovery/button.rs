//! User LEDs

use stm32f3x4::{GPIOA, Gpioa, Rcc};

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
        	(*GPIOA.get()).idr.read().idr0().bits() == 1
		}
    }
}

/// Initializes all the user LEDs
pub fn init(gpioa: &Gpioa, rcc: &Rcc) {
	unsafe {
		// Power up peripherals
		rcc.ahbenr.modify(|_, w| w.iopaen().bits(1));

		// Configure pins 0 as inputs
		gpioa
		    .moder
		    .modify(
		        |_, w| {
		            w.moder0()
		             .bits(0)
		        },
		    );
	}
}

