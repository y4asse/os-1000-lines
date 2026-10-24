use core::arch::naked_asm;
use core::panic::PanicInfo;

pub fn putchar(_ch: u8) {}

#[unsafe(no_mangle)]
pub extern "C" fn exit() -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.start")]
#[unsafe(naked)]
pub extern "C" fn start() -> ! {
    naked_asm!(
        "la sp, __stack_top",
        "call main",
        "call exit",
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit()
}
