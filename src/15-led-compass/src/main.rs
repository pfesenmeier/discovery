#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{
    entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, Measurement,
};
// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303agr, mut delay, mut itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let Measurement { x, y, .. } = lsm303agr.mag_data().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians
        iprintln!(&mut itm.stim[0], "{}", theta);

        // FIXME pick a direction to point to based on `theta`
        let dir = match theta {
            // 22.5 to -22.5
            theta if 0.39 > theta && theta > -0.39 => Direction::North,
            // 22.5 to 67.5
            theta if 1.18 > theta && theta > 0.39 => Direction::Northwest,
            // 112.5 to 67.5
            theta if 1.96 > theta && theta > 1.18 => Direction::West,
            // 112.5 to 157.5
            theta if 2.75 > theta && theta > 1.96 => Direction::Southwest,
            theta if (PI > theta && theta > 2.75) || (-PI < theta && theta < -2.75) => {
                Direction::South
            }
            theta if -2.75 < theta && theta < -1.96 => Direction::Southeast,
            theta if -1.96 < theta && theta < -1.18 => Direction::East,
            theta if -1.18 < theta && theta < -0.39 => Direction::Northeast,
            _ => Direction::North,
        };
        // b/t -1/4 and 1/4
        // b/t 1/4 and 3/4
        // b/t 3/4 and 5/4
        // b/t 5/4 and 7/4
        // b/t 7/4 and -7/4
        // b/t -1/4 and -3/4
        // b/t -3/4 and -5/4
        // b/t -5/4 and -7/4

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(100_u8);
    }
}
