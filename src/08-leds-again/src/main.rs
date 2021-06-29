#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // initialize GPIOE
    rcc.ahbenr.write(|w| {
      w.iopeen().set_bit()
    });

    // configure each pin to be an output
    gpioe.moder.write(|w| {
        w.moder8().bits(0b01);
        w.moder9().bits(0b01);
        w.moder10().bits(0b01);
        w.moder11().bits(0b01);
        w.moder12().bits(0b01);
        w.moder13().bits(0b01);
        w.moder14().bits(0b01);
        w.moder15().bits(0b01)
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
