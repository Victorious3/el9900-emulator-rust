#![allow(non_snake_case)]

use std::rc::Rc;

use crate::core::{Memory, Package};

mod core;

mod TLCS900L1;
mod TMP91C016;

fn main() {
    let mut device = TMP91C016::new();
    let device = Rc::get_mut(&mut device).unwrap();

    device.reset();
    device.write_byte(0x0000_0000, 0x12);
}
