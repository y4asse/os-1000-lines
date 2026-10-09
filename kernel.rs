#![no_std]
#![no_main]

use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;
use core::ptr;

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
}

struct SbiRet {
    error: isize,
    value: isize,
}

fn sbi_call(
    arg0: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    fid: isize,
    eid: isize,
) -> SbiRet {
    let error;
    let value;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error, value }
}

fn putchar(ch: u8) {
    sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1);
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

    putchar(b'A');

    loop {
        unsafe { asm!("wfi") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
