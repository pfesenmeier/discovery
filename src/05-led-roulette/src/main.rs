#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use picorand::{WyRand, RNG};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 100_u8;

    let mut rng = RNG::<WyRand, usize>::new(42);

    loop {
            let next = rng.generate_range(0, 7);
            leds[next].on().ok();
            delay.delay_ms(half_period);
            leds[next].off().ok();
            delay.delay_ms(half_period);
    }
}
