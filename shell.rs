#![no_std]
#![no_main]

#[macro_use]
mod common;
mod user;

use user::putchar;

#[unsafe(no_mangle)]
extern "C" fn main() {
    loop {}
}
