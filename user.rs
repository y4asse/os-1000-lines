use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;

use crate::common::SYS_PUTCHAR;

fn syscall(sysno: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") sysno,
        );
    }
    ret
}

pub fn putchar(ch: u8) {
    syscall(SYS_PUTCHAR, ch as usize, 0, 0);
}

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
