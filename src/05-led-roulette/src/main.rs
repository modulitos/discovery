#![deny(unsafe_code)]
#![no_main]
#![no_std]
// #![feature(panic_impl)]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    // let leds = vec![0, 1, 2];

    let period = 50_u8;

    // infinite loop; just so we don't leave this stack frame
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;
            leds[next].on();
            delay.delay_ms(period);
            leds[curr].off();
            delay.delay_ms(period);
        }
    }
}

// // Original on/off implementation:
// #[entry]
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//     let half_period = 500_u16;

//     // infinite loop; just so we don't leave this stack frame
//     loop {
//         leds[0].on();
//         delay.delay_ms(half_period);

//         leds[0].off();
//         delay.delay_ms(half_period);
//     }
// }
