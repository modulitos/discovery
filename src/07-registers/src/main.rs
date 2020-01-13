#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

// use core::ptr;

// #[entry]
// fn main() -> ! {
//     aux7::init();

//     unsafe {
//         // A magic address!
//         const GPIOE_BSRR: u32 = 0x48001018;

//         // Turn on the "North" LED (red)
//         // *(GPIOE_BSRR as *mut u32) = 1 << 9;
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

//         // Turn on the "East" LED (green)
//         // *(GPIOE_BSRR as *mut u32) = 1 << 11;
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

//         // Turn off the "North" LED
//         // *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

//         // Turn off the "East" LED
//         // *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
//     }

//     loop {}
// }

// #[entry]
// fn main() -> ! {
//     let mut itm = aux7::init().0;

//     unsafe {
//         const GPIOE_BSRR: u32 = 0x4800_1018;
//         const GPIOE_ODR: u32 = 0x4800_1014;

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn on the NORTH LED (red)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn on the EAST LED (green)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn off the NORTH LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn off the EAST LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
//     }

//     loop {}
// }

#[entry]
fn main() -> ! {
    // let gpioe = aux7::init().1;
    let (mut itm, gpioe) = aux7::init();

    // Turn on the North LED
    iprintln!(&mut itm.stim[0], "turning on North LED");
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    iprintln!(&mut itm.stim[0], "turning on EAST LED");
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    iprintln!(&mut itm.stim[0], "turning off North LED");
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    iprintln!(&mut itm.stim[0], "turning off East LED");
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
