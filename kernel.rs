#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
pub extern "C" fn boot() -> ! {
    naked_asm!(
        "la sp, __stack_top",
        "j kernel_main",
    );
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
