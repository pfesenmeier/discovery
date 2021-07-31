#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux99::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (mut bme280, mut delay, mut itm) = aux99::init();

    loop {
        let measurements = bme280.measure(&mut delay).unwrap();
        // iprintln!(&mut itm.stim[0], "Relative Humidity = {}%", measurements.humidity);
        iprintln!(&mut itm.stim[0], "Temperature = {} deg C", measurements.temperature);
        // iprintln!(&mut itm.stim[0], "Pressure = {} pascals", measurements.pressure);
        delay.delay_ms(1000_u16);
    }
}
