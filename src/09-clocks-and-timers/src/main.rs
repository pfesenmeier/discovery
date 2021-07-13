#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    const K: u16 = 3; // this value needs to be tweaked
    for _ in 0..(K * ms) {
        aux9::nop()
    }
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // TODO initialize TIM6
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    let ms = 999;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
