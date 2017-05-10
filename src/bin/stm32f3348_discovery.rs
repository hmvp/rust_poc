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

extern crate rust_poc;

// device crate generated using svd2rust
extern crate stm32f3x4;

extern crate cast;

use rtfm::{Local, P0, P1, T0, T1, TMax};
use rust_poc::stm32f3348_discovery::{led, button};
use stm32f3x4::interrupt::Tim7Dac2;
use rust_poc::{idle, stm32f3348_discovery};

// CONFIGURATION
const FREQUENCY: u32 = 1; // Hz


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
 	stm32f3348_discovery::init(priority, threshold, FREQUENCY)
}

