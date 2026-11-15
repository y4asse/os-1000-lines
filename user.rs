use core::arch::{asm, naked_asm};
use core::ffi::CStr;
use core::panic::PanicInfo;

use crate::common::{SYS_EXIT, SYS_GETCHAR, SYS_PUTCHAR, SYS_READFILE, SYS_WRITEFILE};

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

pub fn getchar() -> isize {
    syscall(SYS_GETCHAR, 0, 0, 0)
}

pub fn readfile(filename: &CStr, buf: &mut [u8]) -> isize {
    syscall(SYS_READFILE, filename.as_ptr().addr(), buf.as_mut_ptr().addr(), buf.len())
}

pub fn writefile(filename: &CStr, buf: &[u8]) -> isize {
    syscall(SYS_WRITEFILE, filename.as_ptr().addr(), buf.as_ptr().addr(), buf.len())
}

#[unsafe(no_mangle)]
pub extern "C" fn exit() -> ! {
    syscall(SYS_EXIT, 0, 0, 0);
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
