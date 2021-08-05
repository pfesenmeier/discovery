#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};
use aux14::i2c1::RegisterBlock;

// slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// magnetometer's registers
const WHO_AM_I_M: u8 = 0x4F;
const CFG_REG_A_M: u8 = 0x60;
const OUT_X_L_REG_M: u8 = 0x068;

fn start_connection(i2c1: &RegisterBlock, address: u16, bytes: u8) {
    i2c1.cr2.write(|w| {
        w.start().set_bit();
        w.sadd().bits(address);
        w.rd_wrn().clear_bit();
        w.nbytes().bits(bytes);
        w.autoend().clear_bit()
    });
}

fn start_read(i2c1: &RegisterBlock, bytes: u8) {
    i2c1.cr2.modify(|_, w| {
        w.start().set_bit();
        w.nbytes().bits(bytes);
        w.rd_wrn().set_bit();
        w.autoend().set_bit()
    });
}

fn set_mode_continuous_lsm303agr(i2c1: &RegisterBlock) -> u8 {
    start_connection(i2c1, MAGNETOMETER, 2);

    while i2c1.isr.read().txis().bit_is_clear() {}
    i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
    i2c1.txdr.write(|w| w.txdata().bits(0x0));

    while i2c1.isr.read().tc().bit_is_clear() {}
    start_read(i2c1, 1);

    while i2c1.isr.read().rxne().bit_is_clear() {}
    i2c1.rxdr.read().rxdata().bits()
}

fn who_am_i_lsm303agr(i2c1: &RegisterBlock) -> u8 {
    start_connection(i2c1, MAGNETOMETER, 1);

    while i2c1.isr.read().txis().bit_is_clear() {}
    i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I_M));

    while i2c1.isr.read().tc().bit_is_clear() {}
    start_read(i2c1, 1);

    while i2c1.isr.read().rxne().bit_is_clear() {}
    i2c1.rxdr.read().rxdata().bits()
}

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    let cfg_reg_a_m_byte: u8 = set_mode_continuous_lsm303agr(i2c1);
    // Expected output:  0x60 - 0b00000000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", CFG_REG_A_M, cfg_reg_a_m_byte);

    let who_am_i: u8 = who_am_i_lsm303agr(i2c1);
    // Expected output:  0x4F - 0b01000000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", WHO_AM_I_M, who_am_i);

    // sample magnetometer
    loop {
        start_connection(i2c1, MAGNETOMETER, 1);

        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_L_REG_M));

        while i2c1.isr.read().tc().bit_is_clear() {}
        start_read(i2c1, 6);

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            // Wait until we have received something
            while i2c1.isr.read().rxne().bit_is_clear() {}

            *byte = i2c1.rxdr.read().rxdata().bits();
        }

        iprintln!(&mut itm.stim[0], "{:?}", buffer);
        delay.delay_ms(1000_u16);
    }
}
