#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const CONFIG_REG_A_M: u8 = 0x60;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    // Stage 0 -> Set peripheral to continuous read
    i2c1.cr2.write(|w| {
        w.start().set_bit();
        w.sadd().bits(MAGNETOMETER);
        w.rd_wrn().clear_bit();
        w.nbytes().bits(2);
        w.autoend().clear_bit()
    });

    // wait until data has been sent
    while i2c1.isr.read().txis().bit_is_clear() {}

    // Send the address of the register that we want to read
    i2c1.txdr.write(|w| { w.txdata().bits(CONFIG_REG_A_M) });
    i2c1.txdr.write(|w| { w.txdata().bits(0x0) });

    // wait until NBYTES of data has been transferred
    while i2c1.isr.read().tc().bit_is_clear() {}

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    loop {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        // Begin Master - Slave Write protocol?
        i2c1.cr2.write(|w| {
            // START
            w.start().set_bit();
            // Slave address
            w.sadd().bits(MAGNETOMETER);
            // Read / Write bit set to write (see write to txdata register)
            w.rd_wrn().clear_bit();
            // Request to transfer 1 BYTE
            w.nbytes().bits(1);
            // Enter software (not automatic) end mode
            w.autoend().clear_bit()
        });

        // wait until data has been sent
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read
        i2c1.txdr.write(|w| {
            w.txdata().bits(OUT_X_H_M)
        });

        // wait until NBYTES of data has been transferred
        while i2c1.isr.read().tc().bit_is_clear() {}


        // Stage 2: Receive the contents of the register we asked for
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            // Broadcast RESTART
            w.start().set_bit();
            w.nbytes().bits(6);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            while i2c1.isr.read().rxne().bit_is_clear() {}

            *byte = i2c1.rxdr.read().rxdata().bits();
        }

        iprintln!(&mut itm.stim[0], "{:?}", buffer);

        delay.delay_ms(1_000_u16);
    }
}