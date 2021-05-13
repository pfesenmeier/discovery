#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u8;

    loop {
        for curr in 0..8 {
            let next: usize = (curr + 1) % 8;
            leds[next].on().ok();
            delay.delay_ms(half_period);

            leds[curr].off().ok();
            delay.delay_ms(half_period);
        }
    }
}

