// TODO: use the stm31 discovery controller as base and the embedded rust book to build core functionality
// TODO: write tests while doing this


// TODO: get the current orientation wrt onboard accelerometer


// everything i need is in the board used in the rust embedded book: https://docs.rust-embedded.org/book/intro/hardware.html

// i think this should be used:https://docs.rust-embedded.org/discovery/microbit/08-led-compass/take-1.html

#![deny(unsafe_code)]
#![no_main]
#![no_std]


use rtt_target::{rprintln, rtt_init_print};

fn main() {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

}
