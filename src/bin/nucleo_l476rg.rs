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
extern crate stm32l4x6;

extern crate cast;

use rtfm::{Local, P0, P1, T0, T1, TMax};
use rust_poc::nucleo_l476rg::{led, button};
use stm32l4x6::interrupt::Tim7;
use rust_poc::{idle, nucleo_l476rg};

// CONFIGURATION
const FREQUENCY: u32 = 1; // Hz


// TASKS (None in this example)
tasks!(stm32l4x6, {
	periodic: Task {
        interrupt: Tim7,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim7, _priority: P1, _threshold: T1) {
    //asm::bkpt();

    // Task local data
    static INDEX: Local<usize, Tim7> = Local::new(0);

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

fn next_led(index: usize) -> usize {
	if index == 0 {
		&led::LEDS[0].off();
		1
	} else {
		&led::LEDS[0].on();
		0
	}
}

// INITIALIZATION PHASE
fn init(priority: P0, threshold: &TMax) {
    hprintln!("INIT");
 	nucleo_l476rg::init(priority, threshold, FREQUENCY)
}

