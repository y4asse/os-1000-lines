#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;
use core::ptr;

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
}

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
    unsafe {
        let start = &raw mut __bss;
        let end = &raw mut __bss_end;
        ptr::write_bytes(start, 0, end.addr() - start.addr());
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
