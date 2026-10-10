#![no_std]
#![no_main]

#[macro_use]
mod common;

macro_rules! read_csr {
    ($csr:literal) => {{
        let value: usize;
        unsafe { core::arch::asm!(concat!("csrr {}, ", $csr), out(reg) value) };
        value
    }};
}

macro_rules! write_csr {
    ($csr:literal, $value:expr) => {{
        let value: usize = $value;
        unsafe { core::arch::asm!(concat!("csrw ", $csr, ", {}"), in(reg) value) };
    }};
}

use core::arch::{asm, global_asm, naked_asm};
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

use common::{PAGE_SIZE, PAddr};

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    static mut __free_ram: u8;
    static mut __free_ram_end: u8;
}

static NEXT_OFFSET: AtomicUsize = AtomicUsize::new(0);

fn alloc_pages(n: usize) -> PAddr {
    let size = n * PAGE_SIZE;
    let offset = NEXT_OFFSET.fetch_add(size, Ordering::Relaxed);
    unsafe {
        let start = (&raw mut __free_ram).addr();
        let end = (&raw mut __free_ram_end).addr();
        let paddr = start + offset;
        if paddr + size > end {
            panic!("out of memory");
        }
        ptr::write_bytes(paddr as *mut u8, 0, size);
        paddr
    }
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

#[repr(C)]
struct TrapFrame {
    ra: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    sp: u32,
}

global_asm!(
    ".balign 4",
    ".global kernel_entry",
    "kernel_entry:",
    "csrw sscratch, sp",
    "addi sp, sp, -4 * 31",
    "sw ra,  4 * 0(sp)",
    "sw gp,  4 * 1(sp)",
    "sw tp,  4 * 2(sp)",
    "sw t0,  4 * 3(sp)",
    "sw t1,  4 * 4(sp)",
    "sw t2,  4 * 5(sp)",
    "sw t3,  4 * 6(sp)",
    "sw t4,  4 * 7(sp)",
    "sw t5,  4 * 8(sp)",
    "sw t6,  4 * 9(sp)",
    "sw a0,  4 * 10(sp)",
    "sw a1,  4 * 11(sp)",
    "sw a2,  4 * 12(sp)",
    "sw a3,  4 * 13(sp)",
    "sw a4,  4 * 14(sp)",
    "sw a5,  4 * 15(sp)",
    "sw a6,  4 * 16(sp)",
    "sw a7,  4 * 17(sp)",
    "sw s0,  4 * 18(sp)",
    "sw s1,  4 * 19(sp)",
    "sw s2,  4 * 20(sp)",
    "sw s3,  4 * 21(sp)",
    "sw s4,  4 * 22(sp)",
    "sw s5,  4 * 23(sp)",
    "sw s6,  4 * 24(sp)",
    "sw s7,  4 * 25(sp)",
    "sw s8,  4 * 26(sp)",
    "sw s9,  4 * 27(sp)",
    "sw s10, 4 * 28(sp)",
    "sw s11, 4 * 29(sp)",
    "csrr a0, sscratch",
    "sw a0, 4 * 30(sp)",
    "mv a0, sp",
    "call handle_trap",
    "lw ra,  4 * 0(sp)",
    "lw gp,  4 * 1(sp)",
    "lw tp,  4 * 2(sp)",
    "lw t0,  4 * 3(sp)",
    "lw t1,  4 * 4(sp)",
    "lw t2,  4 * 5(sp)",
    "lw t3,  4 * 6(sp)",
    "lw t4,  4 * 7(sp)",
    "lw t5,  4 * 8(sp)",
    "lw t6,  4 * 9(sp)",
    "lw a0,  4 * 10(sp)",
    "lw a1,  4 * 11(sp)",
    "lw a2,  4 * 12(sp)",
    "lw a3,  4 * 13(sp)",
    "lw a4,  4 * 14(sp)",
    "lw a5,  4 * 15(sp)",
    "lw a6,  4 * 16(sp)",
    "lw a7,  4 * 17(sp)",
    "lw s0,  4 * 18(sp)",
    "lw s1,  4 * 19(sp)",
    "lw s2,  4 * 20(sp)",
    "lw s3,  4 * 21(sp)",
    "lw s4,  4 * 22(sp)",
    "lw s5,  4 * 23(sp)",
    "lw s6,  4 * 24(sp)",
    "lw s7,  4 * 25(sp)",
    "lw s8,  4 * 26(sp)",
    "lw s9,  4 * 27(sp)",
    "lw s10, 4 * 28(sp)",
    "lw s11, 4 * 29(sp)",
    "lw sp,  4 * 30(sp)",
    "sret",
);

unsafe extern "C" {
    fn kernel_entry();
}

#[unsafe(no_mangle)]
extern "C" fn handle_trap(_f: &mut TrapFrame) {
    let scause = read_csr!("scause");
    let stval = read_csr!("stval");
    let user_pc = read_csr!("sepc");

    panic!("unexpected trap scause={:08x}, stval={:08x}, sepc={:08x}", scause, stval, user_pc);
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

    write_csr!("stvec", (kernel_entry as *const ()).addr());

    let paddr0 = alloc_pages(2);
    let paddr1 = alloc_pages(1);
    printf!("alloc_pages test: paddr0={:x}\n", paddr0);
    printf!("alloc_pages test: paddr1={:x}\n", paddr1);

    panic!("booted!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(loc) => printf!("PANIC: {}:{}: {}\n", loc.file(), loc.line(), info.message()),
        None => printf!("PANIC: {}\n", info.message()),
    }
    loop {}
}
