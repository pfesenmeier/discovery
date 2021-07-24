#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        // TODO Receive a user request. Each user request ends with ENTER
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let byte = usart1.rdr.read().rdr().bits() as u8;
        // if byte != '\n'
        if byte == 10 {
          print_bytes(&buffer)
        } else {
           
        }
        // push on buffer
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        /*
          reverse string
          print out string byte by byte
         */
        // TODO Send back the reversed string
    }
}

fn print_bytes(_bytes: &Vec<u8, 32>) -> () {

}


