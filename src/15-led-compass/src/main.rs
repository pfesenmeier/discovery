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

        // angle(0) is dead North, angle(4) is West, angle(-2) is Northeast, and so on
        fn angle(num: i8) -> f32 {
            num as f32 / 8. * PI
        }

        let is_between = |smaller, bigger| {
            theta > angle(smaller) && theta < angle(bigger)
        };

        let dir = match theta {
            _theta if is_between(-7, -5) => Direction::Southeast,
            _theta if is_between(-5, -3) => Direction::East,
            _theta if is_between(-3, -1) => Direction::Northeast,
            _theta if is_between(-1, 1) => Direction::North,
            _theta if is_between(1, 3) => Direction::Northwest,
            _theta if is_between(3, 5) => Direction::West,
            _theta if is_between(5, 7) => Direction::Southwest,
            _ => Direction::South,
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(10_u8);
    }
}
