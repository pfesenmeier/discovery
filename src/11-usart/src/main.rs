#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        // TODO Receive a user request. Each user request ends with ENTER
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let byte = usart1.rdr.read().rdr().bits() as u8;
        match byte {
            // '\r'
            13 => {
                buffer.reverse();
                for byte in &buffer {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }
                buffer.clear();
            }
            _ => {
                if let Err(x) = buffer.push(byte) {
                    iprintln!(&mut itm.stim[0], "Error while pushing '{}' to array", x);
                }
            }
        }
    }
    /*
      reverse string
      print out string byte by byte
     */
    // TODO Send back the reversed string
}


