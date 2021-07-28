#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const _OUT_X_H_M: u8 = 0x03;
const WHO_AM_I_M: u8 = 0x4F;
const EXPECTED_VALUE: u8 = 0b01000000;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| {
            w.txdata().bits(WHO_AM_I_M)
        });

        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().clear_bit();
            w.autoend().clear_bit()
        });

        while i2c1.isr.read().rxne().bit_is_clear() {}

        i2c1.rxdr.read().rxdata().bits()
    };

    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", WHO_AM_I_M, byte);
    iprintln!(&mut itm.stim[0], "expected value: 0b{:08b}", EXPECTED_VALUE);

    loop {}
}