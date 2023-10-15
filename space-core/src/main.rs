#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
// Just use the HAL for pulling in its interrupt vectors (and may be some other
// fairy dust).
use stm32f3xx_hal as _;

// to run:
// in one terminal:
//  openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg 
// in the other terminal:
// gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette


#[entry]
fn main() -> ! {
    let _y;
    let x = 40;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}