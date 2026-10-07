#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
pub extern "C" fn boot() -> ! {
    naked_asm!("j boot");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
