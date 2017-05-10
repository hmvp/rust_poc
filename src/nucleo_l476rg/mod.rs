pub mod led;
pub mod timer;
pub mod button;

use rtfm::{P0, TMax};

// RESOURCES
peripherals!(stm32l4x6, {
	GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C0,
    },
    GPIOC: Peripheral {
        register_block: Gpioc,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
	TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});

pub fn init(priority: P0, threshold: &TMax, frequency: u32) {
 	let gpioa = GPIOA.access(&priority, threshold);
 	let gpioc = GPIOC.access(&priority, threshold);
    let rcc = RCC.access(&priority, threshold);
	let tim7 = TIM7.access(&priority, threshold);
    let timer = timer::Timer(&*tim7);
	timer.init(&rcc, frequency);

	// Configure the PAx pins as input pins
    button::init(&gpioc, &rcc);

	// Configure the PBx pins as output pins
    led::init(&gpioa, &rcc);

	// Configure TIM7 for periodic update events
    timer.init(&rcc, frequency);

    // Start the timer
    timer.resume();
}
