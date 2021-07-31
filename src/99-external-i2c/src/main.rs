#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux99::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (_leds, mut lsm303agr, mut delay, mut itm) = aux99::init();

    loop {
        iprintln!(&mut itm.stim[0], "{:?}", lsm303agr.mag_data().unwrap());
        delay.delay_ms(1_000_u16);
    }
}
