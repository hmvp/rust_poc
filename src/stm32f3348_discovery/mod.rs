pub mod led;
pub mod timer;
pub mod button;

use rtfm::{P0, TMax};
use rtfm;

// RESOURCES
peripherals!(stm32f3x4, {
	/* GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C0,
    }, */
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C0,
    },
    /* RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
	TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    }, */
});

#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIOA_2: rtfm::Peripheral<::stm32f3x4::Gpioa, rtfm::C0> =
	unsafe { rtfm::Peripheral::_new(::stm32f3x4::GPIOA) };

#[allow(private_no_mangle_statics)]
#[no_mangle]
static RCC_2: rtfm::Peripheral<::stm32f3x4::Rcc, rtfm::C0> =
	unsafe { rtfm::Peripheral::_new(::stm32f3x4::RCC) };

#[allow(private_no_mangle_statics)]
#[no_mangle]
static TIM7_2: rtfm::Peripheral<::stm32f3x4::Tim7, rtfm::C1> =
	unsafe { rtfm::Peripheral::_new(::stm32f3x4::TIM7) };


pub fn init(priority: P0, threshold: &TMax, frequency: u32) {
 	let gpioa = GPIOA_2.access(&priority, threshold);
 	let gpiob = GPIOB.access(&priority, threshold);
    let rcc = RCC_2.access(&priority, threshold);
	let tim7 = TIM7_2.access(&priority, threshold);
    let timer = timer::Timer(&*tim7);
	timer.init(&rcc, frequency);

	// Configure the PAx pins as input pins
    button::init(&gpioa, &rcc);

	// Configure the PBx pins as output pins
    led::init(&gpiob, &rcc);

	// Configure TIM7 for periodic update events
    timer.init(&rcc, frequency);

    // Start the timer
    timer.resume();
}

