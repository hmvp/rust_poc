//! Prints "Hello, world!" on the OpenOCD console using semihosting
#![feature(const_fn)]
#![feature(used)]
#![no_std]

#[macro_use] // for the `hprintln!` macro
extern crate cortex_m;

// before main initialization + `start` lang item
extern crate cortex_m_rt;

#[macro_use] // for the `tasks!` macro
extern crate cortex_m_rtfm as rtfm;

// device crate generated using svd2rust
extern crate stm32f3x4;

extern crate cast;

mod led;
mod timer;
mod button;

use cortex_m::asm;
use rtfm::{Local, P0, P1, T0, T1, TMax};
use stm32f3x4::interrupt::Tim7Dac2;

// CONFIGURATION
const FREQUENCY: u32 = 1; // Hz

// RESOURCES
peripherals!(stm32f3x4, {
	GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C0,
    },
    GPIOB: Peripheral {
        register_block: Gpiob,
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

// TASKS (None in this example)
tasks!(stm32f3x4, {
	periodic: Task {
        interrupt: Tim7Dac2,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim7Dac2, _priority: P1, _threshold: T1) {
    //asm::bkpt();

    // Task local data
    static INDEX: Local<usize, Tim7Dac2> = Local::new(0);

    let index = INDEX.borrow_mut(&mut task);

	if button::BUTTONS[0].state() {
		for led in &led::LEDS {
			led.off();		
		}

		*index = 0
	}

	hprintln!("{}", index);

	*index = next_led(*index);
}

fn next_led(mut index: usize) -> usize {
	&led::LEDS[index].off();

	index += 1;

	if index > 3 {
		index = 0
	}

	&led::LEDS[index].on();

	index
}

// INITIALIZATION PHASE
fn init(priority: P0, threshold: &TMax) {
    hprintln!("INIT");
 	let gpioa = GPIOA.access(&priority, threshold);
 	let gpiob = GPIOB.access(&priority, threshold);
    let rcc = RCC.access(&priority, threshold);
	let tim7 = TIM7.access(&priority, threshold);
    let timer = timer::Timer(&*tim7);
	timer.init(&rcc, FREQUENCY);

	// Configure the PAx pins as input pins
    button::init(&gpioa, &rcc);

	// Configure the PBx pins as output pins
    led::init(&gpiob, &rcc);

	// Configure TIM7 for periodic update events
    timer.init(&rcc, FREQUENCY);

    // Start the timer
    timer.resume();
}

// IDLE LOOP
fn idle(_priority: P0, _threshold: T0) -> ! {
    hprintln!("IDLE");

    // Sleep
    loop {
        rtfm::wfi();
    }
}

