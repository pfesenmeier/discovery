#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux97::{entry, iprint, iprintln, prelude::*};
use sht3x::{Address,Repeatability,SHT3x};

#[entry]
fn main() -> ! {
    let (i2c, mut delay, mut itm) = aux97::init();
    let mut sht3x = SHT3x::new(i2c, Address::Low);

    loop {
        let measurements = sht3x.measure(Repeatability::High, &mut delay).unwrap();
        let celsius: f64 = measurements.temperature as f64 / 100.0;
        let farenheit = (celsius * 9.0 / 5.0) + 32.0;

        iprintln!(&mut itm.stim[0], "{:.2} ", farenheit);
        delay.delay_ms(1000_u16);
    }
}
