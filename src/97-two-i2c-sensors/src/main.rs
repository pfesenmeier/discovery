#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux97::{entry, iprint, iprintln, prelude::*};
use sht3x::{Address,Repeatability,SHT3x};
use bme280::BME280;
use shared_bus;

#[entry]
fn main() -> ! {
    let (i2c, mut delay, mut itm) = aux97::init();
    let bus = shared_bus::BusManagerSimple::new(i2c);
    let mut bme280 = BME280::new_primary(bus.acquire_i2c());
    bme280.init(&mut delay).unwrap();
    let mut sht3x = SHT3x::new(bus.acquire_i2c(), Address::Low);

    loop {
        let sht3x_measurements = sht3x.measure(Repeatability::High, &mut delay).unwrap();
        iprintln!(&mut itm.stim[0], "SHT31D: {:.2} deg F", c_to_f(sht3x_measurements.temperature as f64 / 100.0));

        let bme280_measurements = bme280.measure(&mut delay).unwrap();
        iprintln!(&mut itm.stim[0], "BME280: {:.2} deg F", c_to_f(bme280_measurements.temperature as f64));

        delay.delay_ms(1000_u16);
    }
}

fn c_to_f (celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
