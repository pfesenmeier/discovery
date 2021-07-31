#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux98::{entry, iprint, iprintln, prelude::*};
use sht3x::{Address,Repeatability,SHT3x};

#[entry]
fn main() -> ! {
    let (i2c, mut delay, mut itm) = aux98::init();
    let mut sht3x = SHT3x::new(i2c, Address::Low);

    loop {
        let measurements = sht3x.measure(Repeatability::High, &mut delay).unwrap();
        // iprintln!(&mut itm.stim[0], "Relative Humidity = {}%", measurements.humidity);
        iprintln!(&mut itm.stim[0], "Temperature = {} deg C", measurements.temperature);
        // iprintln!(&mut itm.stim[0], "Pressure = {} pascals", measurements.pressure);
        delay.delay_ms(1000_u16);
    }
}
