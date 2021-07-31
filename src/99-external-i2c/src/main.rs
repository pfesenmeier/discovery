#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux99::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (mut bme280, mut itm) = aux99::init();

        let measurements = bme280.measure().unwrap();
        iprintln!(&mut itm.stim[0], "Relative Humidity = {}%", measurements.humidity);
        iprintln!(&mut itm.stim[0], "Temperature = {} deg C", measurements.temperature);
        iprintln!(&mut itm.stim[0], "Pressure = {} pascals", measurements.pressure);
    loop {}
}
