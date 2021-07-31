//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    leds::Leds,
    lsm303dlhc::I16x3,
    stm32f3xx_hal::{delay::Delay, prelude, stm32::i2c1},
    switch_hal,
};

use stm32f3_discovery::{
    stm32f3xx_hal::{
        gpio::gpiob::{PB6, PB7},
        gpio::AF4,
        i2c::I2c,
        prelude::*,
        stm32::{self, I2C1},
    },
};
pub use lsm303agr:: { UnscaledMeasurement};
use lsm303agr::{interface::I2cInterface, mode, Lsm303agr, MagOutputDataRate};
use bme280::BME280;

pub type Bme280 = BME280<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>, Delay>;

/// Cardinal directions. Each one matches one of the user LEDs.
pub enum Direction {
    /// North / LD3
    North,
    /// Northeast / LD5
    Northeast,
    /// East / LD7
    East,
    /// Southeast / LD9
    Southeast,
    /// South / LD10
    South,
    /// Southwest / LD8
    Southwest,
    /// West / LD6
    West,
    /// Northwest / LD4
    Northwest,
}

pub fn init() -> (Bme280, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    let delay = Delay::new(cp.SYST, clocks);

    let mut bme280 = BME280::new_primary(i2c, delay);

    bme280.init().unwrap();

    (bme280, cp.ITM)
}
