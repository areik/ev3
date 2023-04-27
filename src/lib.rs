#![no_std]
#![feature(lang_items, core_intrinsics)]
#![allow(dead_code, improper_ctypes, unused_imports)]

pub mod ev3rt;

extern crate rcstring;
use core::intrinsics;
use core::panic::PanicInfo;
use rcstring::*;

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
    panic!()
}
