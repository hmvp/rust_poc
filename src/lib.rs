//! Prints "Hello, world!" on the OpenOCD console using semihosting
#![feature(const_fn)]
#![feature(used)]
#![no_std]

#[macro_use] // for the `hprintln!` macro
extern crate cortex_m;

#[macro_use] // for the `tasks!` macro
extern crate cortex_m_rtfm as rtfm;

// device crate generated using svd2rust
pub extern crate stm32l4x6;
pub extern crate stm32f3x4;

extern crate cast;

pub mod nucleo_l476rg;
pub mod stm32f3348_discovery;

use rtfm::{P0, T0};

// IDLE LOOP
pub fn idle(_priority: P0, _threshold: T0) -> ! {
    hprintln!("IDLE");

    // Sleep
    loop {
        rtfm::wfi();
    }
}
