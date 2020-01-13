#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::convert::TryFrom;
// use core::fmt::{self, Write};

#[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln, usart1};
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};

// macro_rules! uprint {
//     ($serial:expr, $($arg:tt)*) => {
//         $serial.write_fmt(format_args!($($arg)*)).ok()
//     };
// }

// macro_rules! uprintln {
//     ($serial:expr, $fmt:expr) => {
//         uprint!($serial, concat!($fmt, "\n"))
//     };
//     ($serial:expr, $fmt:expr, $($arg:tt)*) => {
//         uprint!($serial, concat!($fmt, "\n"), $($arg)*)
//     };
// }

// struct SerialPort {
//     usart1: &'static mut usart1::RegisterBlock,
// }

// impl fmt::Write for SerialPort {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         // for rdr_val in b"The quick brown fox jumps over the lazy dog.".iter() {
//         for rdr_val in s.as_bytes() {
//             // wait until it's safe to write to TDR
//             while self.usart1.isr.read().txe().bit_is_clear() {}

//             self.usart1.tdr.write(|w| w.tdr().bits(u16::from(*rdr_val)));
//         }
//         Ok(())
//     }
// }

// echo server with a reversed string implementation:

#[entry]
fn main() -> ! {
    // let (usart1, mono_timer, itm) = aux11::init();

    let (usart1, mono_timer, mut itm) = aux11::init();

    // let brr_div = usart1.brr.read().bits();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    iprintln!(&mut itm.stim[0], "starting!!!");
    loop {
        let over8_is_set = usart1.cr1.read().over8().bit_is_set();
        iprintln!(&mut itm.stim[0], "over8: {}", over8_is_set);
        let brr_div = usart1.brr.read().bits();
        iprintln!(&mut itm.stim[0], "brr_div: {:0x}", brr_div + 10);

        iprintln!(&mut itm.stim[0], "outer loop - clearing buffer");
        buffer.clear();
        loop {
            iprintln!(&mut itm.stim[0], "inside inner loop");
            // let over8 = usart1.cr1.read().over8().bit();

            while usart1.isr.read().rxne().bit_is_clear() {}
            iprintln!(
                &mut itm.stim[0],
                "isr (interrupt and status register) is clear!"
            );

            let byte = usart1.rdr.read().rdr().bits() as u8;
            iprintln!(&mut itm.stim[0], "byte: {}", char::from(byte));

            if buffer.push(byte).is_err() {
                // buffer full
                iprintln!(&mut itm.stim[0], "error pushing byte onto buffer");
                for byte in b"error: buffer full\n\r" {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }

            // Carriage return
            if byte == 13 {
                iprintln!(&mut itm.stim[0], "byte is 13");
                // Respond
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }
        }
    }
}

// #[entry]
// fn main() -> ! {
//     let (usart1, _mono_timer, mut itm) = aux11::init();
//     // let (usart1, _mono_timer, _itm) = aux11::init();

//     // A buffer with 32 bytes of capacity
//     let mut buffer: Vec<u8, consts::U32> = Vec::new();
//     // NOTE: ISR's rxne bit is getting stuck when using iprintlin!
//     // Disabling for now...
//     // iprintln!(&mut itm.stim[0], "starting!!!");

//     loop {
//         buffer.clear();

//         // Receive a user request. Each user request ends with ENTER
//         loop {
//             iprintln!(&mut itm.stim[0], "buffer clear, cycling inner loop");
//             while usart1.isr.read().rxne().bit_is_clear() {}
//             iprintln!(&mut itm.stim[0], "isr (interrupt and status register) is clear!");
//             // let byte = u8::try_from(usart1.rdr.read().rdr().bits())
//             //     .expect("unable to get u8 from rdr_val input");
//             // if let Ok(byte) = u8::try_from(usart1.rdr.read().rdr().bits()) {
//             let byte = u8::try_from(usart1.rdr.read().rdr().bits())
//                 .expect("input larger than a u8 is not handled");
//             // NOTE `buffer.push` returns a `Result`. Handle the error by responding
//             // with an error message.
//             // if let Err(err) = buffer.push(rdr_val as u8) {
//             // let byte = u8::try_from(rdr_val).expect("unable to get u8 from rdr_val input");

//             // usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)));
//             // usart1.tdr.write(|w| w.tdr().bits(u16::from(b'\n')));
//             iprintln!(&mut itm.stim[0], "byte: {}", char::from(byte));
//             if buffer.push(byte).is_err() {
//                 for byte in b"error: buffer full\n\r" {
//                     // for char in "error! unable to add rdr_val to buffer".chars() {
//                     // wait until it's safe to write to TDR
//                     while usart1.isr.read().txe().bit_is_clear() {}
//                     usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//                 }
//                 break;
//             }

//             if byte == 13 {
//                 iprintln!(&mut itm.stim[0], "enter key detected");
//                 // Send back the reversed string
//                 // for rdr_val in buffer.iter().rev().chain(&[b'\n', b'\r', 13]) {
//                 for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
//                     // iprintln!(
//                     //     &mut itm.stim[0],
//                     //     "transmitting rdr_val to serializer: {}",
//                     //     rdr_val
//                     // );
//                     // wait until it's safe to write to TDR
//                     while usart1.isr.read().txe().bit_is_clear() {}
//                     usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//                 }
//                 break;
//             }
//             // } else {
//             //     // iprintln!(&mut itm.stim[0], "unable to get next byte from rdr!!!");
//             //     for byte in b"error: invalid byte\n\r" {
//             //         // for char in "error! unable to add rdr_val to buffer".chars() {
//             //         // wait until it's safe to write to TDR
//             //         while usart1.isr.read().txe().bit_is_clear() {}
//             //         usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//             //     }
//             // }
//         }
//     }
// }

// // Echo server implementation:
// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, itm) = aux11::init();
//     // let mut serial = SerialPort { usart1: &mut usart1 };

//     loop {
//         // Wait until there's data available
//         while usart1.isr.read().rxne().bit_is_clear() {}

//         // Retrieve the data

//         // TODO: Shouldn't we always use something like
//         // `u8::from(bits)`? But I suppose there is no easy way to do
//         // wthat without risking truncation...
//         let rdr_val = usart1.rdr.read().rdr().bits();

//         // let s = core::str::from(rdr_val);
//         usart1.tdr.write(|w| w.tdr().bits(u16::from(rdr_val)));
//         // uprintln!(serial, "test");

//         // aux11::bkpt();
//     }
// }

// with new uprintln macro:
// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, itm) = aux11::init();

//     let mut serial = SerialPort { usart1 };

//     uprintln!(serial, "The answer is {}", 40 + 2);

//     loop {}
// }

// old - "printing" by writing directly to the tdr ("transfer data rdr_val")

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, mut itm) = aux11::init();

//     // Send a single character
//     // usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

//     let instant = mono_timer.now();
//     // Send a string
//     for rdr_val in b"The quick brown fox jumps over the lazy dog.".iter() {
//         // wait until it's safe to write to TDR
//         while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

//         usart1.tdr.write(|w| w.tdr().bits(u16::from(*rdr_val)));
//     }
//     let elapsed = instant.elapsed(); // in ticks

//     iprintln!(
//         &mut itm.stim[0],
//         "`for` loop took {} ticks ({} us)",
//         elapsed,
//         elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
//     );

//     loop {}
// }
