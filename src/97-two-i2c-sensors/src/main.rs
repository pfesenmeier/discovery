#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux97::{entry, iprint, iprintln, prelude::*};
use sht3x::{Address, Repeatability, SHT3x};
use bme280::BME280;
use aux97::OutputSwitch;

#[entry]
fn main() -> ! {
    let (i2c, mut delay, mut leds, mut itm) = aux97::init();
    let bus = shared_bus::BusManagerSimple::new(i2c);
    let mut bme280 = BME280::new_primary(bus.acquire_i2c());
    bme280.init(&mut delay).unwrap();
    let mut sht3x = SHT3x::new(bus.acquire_i2c(), Address::Low);

    loop {
        leds[0].on().ok();
        let sht3x_measurements = sht3x.measure(Repeatability::High, &mut delay).unwrap();
        iprintln!(&mut itm.stim[0], "SHT31D: {:.2} deg F, {:.2}% humidity", c_to_f(sht3x_measurements.temperature as f64 / 100.0), sht3x_measurements.humidity as f64 / 100.0);

        let bme280_measurements = bme280.measure(&mut delay).unwrap();
        iprintln!(&mut itm.stim[0], "BME280: {:.2} deg F, {:.2}% humidity", c_to_f(bme280_measurements.temperature as f64), bme280_measurements.humidity);
        leds[0].off().ok();

        delay.delay_ms(1000_u16);
    }
}

fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
