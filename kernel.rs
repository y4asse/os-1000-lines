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

use common::{PAGE_SIZE, PAddr, VAddr, is_aligned};

unsafe extern "C" {
    static _binary_shell_bin_start: u8;
    static _binary_shell_bin_size: u8;
    static mut __kernel_base: u8;
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
    "csrrw sp, sscratch, sp",
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
    "addi a0, sp, 4 * 31",
    "csrw sscratch, a0",
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

const PROCS_MAX: usize = 8;
const KERNEL_STACK_SIZE: usize = 8192;

#[derive(Clone, Copy, PartialEq)]
enum ProcState {
    Unused,
    Runnable,
}

#[repr(align(16))]
struct KernelStack([u8; KERNEL_STACK_SIZE]);

struct Process {
    pid: usize,
    state: ProcState,
    sp: VAddr,
    page_table: PAddr,
    stack: KernelStack,
}

impl Process {
    const fn new() -> Self {
        Process {
            pid: 0,
            state: ProcState::Unused,
            sp: 0,
            page_table: 0,
            stack: KernelStack([0; KERNEL_STACK_SIZE]),
        }
    }
}

static mut PROCS: [Process; PROCS_MAX] = [const { Process::new() }; PROCS_MAX];

#[unsafe(naked)]
unsafe extern "C" fn switch_context(prev_sp: *mut VAddr, next_sp: *const VAddr) {
    naked_asm!(
        "addi sp, sp, -13 * 4",
        "sw ra,  0  * 4(sp)",
        "sw s0,  1  * 4(sp)",
        "sw s1,  2  * 4(sp)",
        "sw s2,  3  * 4(sp)",
        "sw s3,  4  * 4(sp)",
        "sw s4,  5  * 4(sp)",
        "sw s5,  6  * 4(sp)",
        "sw s6,  7  * 4(sp)",
        "sw s7,  8  * 4(sp)",
        "sw s8,  9  * 4(sp)",
        "sw s9,  10 * 4(sp)",
        "sw s10, 11 * 4(sp)",
        "sw s11, 12 * 4(sp)",
        "sw sp, (a0)",
        "lw sp, (a1)",
        "lw ra,  0  * 4(sp)",
        "lw s0,  1  * 4(sp)",
        "lw s1,  2  * 4(sp)",
        "lw s2,  3  * 4(sp)",
        "lw s3,  4  * 4(sp)",
        "lw s4,  5  * 4(sp)",
        "lw s5,  6  * 4(sp)",
        "lw s6,  7  * 4(sp)",
        "lw s7,  8  * 4(sp)",
        "lw s8,  9  * 4(sp)",
        "lw s9,  10 * 4(sp)",
        "lw s10, 11 * 4(sp)",
        "lw s11, 12 * 4(sp)",
        "addi sp, sp, 13 * 4",
        "ret",
    );
}

const SATP_SV32: usize = 1 << 31;
const PAGE_V: u32 = 1 << 0; // 有効化ビット
const PAGE_R: u32 = 1 << 1; // 読み込み可能
const PAGE_W: u32 = 1 << 2; // 書き込み可能
const PAGE_X: u32 = 1 << 3; // 実行可能
const PAGE_U: u32 = 1 << 4; // ユーザーモードでアクセス可能

unsafe fn map_page(table1: PAddr, vaddr: VAddr, paddr: PAddr, flags: u32) {
    if !is_aligned(vaddr, PAGE_SIZE) {
        panic!("unaligned vaddr {:x}", vaddr);
    }
    if !is_aligned(paddr, PAGE_SIZE) {
        panic!("unaligned paddr {:x}", paddr);
    }

    let table1 = table1 as *mut u32;
    let vpn1 = (vaddr >> 22) & 0x3ff; // 上位10ビット（VPN[1]）を取得する。0x3ffは10ビットのマスク
    unsafe {
        if *table1.add(vpn1) & PAGE_V == 0 {
            let pt_paddr = alloc_pages(1);
            *table1.add(vpn1) = ((pt_paddr / PAGE_SIZE) << 10) as u32 | PAGE_V;
        }

        let vpn0 = (vaddr >> 12) & 0x3ff;
        let table0 = ((*table1.add(vpn1) >> 10) as usize * PAGE_SIZE) as *mut u32;
        *table0.add(vpn0) = ((paddr / PAGE_SIZE) << 10) as u32 | flags | PAGE_V;
    }
}

const USER_BASE: VAddr = 0x1000000;
const SSTATUS_SPIE: usize = 1 << 5;

#[unsafe(naked)]
extern "C" fn user_entry() -> ! {
    naked_asm!(
        "li t0, {sepc}",
        "csrw sepc, t0",
        "li t0, {sstatus}",
        "csrw sstatus, t0",
        "sret",
        sepc = const USER_BASE,
        sstatus = const SSTATUS_SPIE,
    );
}

fn create_process(image: &[u8]) -> usize {
    unsafe {
        let procs = &raw mut PROCS;
        let Some(i) = (0..PROCS_MAX).find(|&i| (*procs)[i].state == ProcState::Unused) else {
            panic!("no free process slots");
        };
        let proc = &mut (*procs)[i];

        let mut sp = proc.stack.0.as_mut_ptr().add(KERNEL_STACK_SIZE).cast::<u32>();
        for _ in 0..12 {
            sp = sp.sub(1);
            *sp = 0;
        }
        sp = sp.sub(1);
        *sp = (user_entry as *const ()).addr() as u32;

        let page_table = alloc_pages(1);
        let kernel_base = (&raw mut __kernel_base).addr();
        let free_ram_end = (&raw mut __free_ram_end).addr();
        for paddr in (kernel_base..free_ram_end).step_by(PAGE_SIZE) {
            map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
        }

        for (i, chunk) in image.chunks(PAGE_SIZE).enumerate() {
            let page = alloc_pages(1);
            ptr::copy_nonoverlapping(chunk.as_ptr(), page as *mut u8, chunk.len());
            map_page(page_table, USER_BASE + i * PAGE_SIZE, page, PAGE_U | PAGE_R | PAGE_W | PAGE_X);
        }

        proc.pid = i + 1;
        proc.state = ProcState::Runnable;
        proc.sp = sp.addr();
        proc.page_table = page_table;
        i
    }
}

fn proc_sp(i: usize) -> *mut VAddr {
    unsafe { &raw mut PROCS[i].sp }
}

static CURRENT_PROC: AtomicUsize = AtomicUsize::new(0);
static IDLE_PROC: AtomicUsize = AtomicUsize::new(0);

fn yield_now() {
    let current = CURRENT_PROC.load(Ordering::Relaxed);
    let next = unsafe {
        let procs = &raw const PROCS;
        let current_pid = (*procs)[current].pid;
        (0..PROCS_MAX)
            .map(|i| (current_pid + i) % PROCS_MAX)
            .find(|&i| (*procs)[i].state == ProcState::Runnable && (*procs)[i].pid > 0)
            .unwrap_or(IDLE_PROC.load(Ordering::Relaxed))
    };

    if next == current {
        return;
    }

    let satp = SATP_SV32 | unsafe { PROCS[next].page_table } / PAGE_SIZE;
    unsafe { 
        asm!(
            "sfence.vma", // TLBの同期
            "csrw satp, {}", // satp レジスタに
            "sfence.vma", // TLBの同期
            in(reg) satp
        )
    };

    let stack_top = unsafe { (&raw mut PROCS[next].stack).cast::<u8>().add(KERNEL_STACK_SIZE) };
    write_csr!("sscratch", stack_top.addr());

    CURRENT_PROC.store(next, Ordering::Relaxed);
    unsafe { switch_context(proc_sp(current), proc_sp(next)) };
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

    let idle = create_process(&[]);
    unsafe { PROCS[idle].pid = 0 };
    IDLE_PROC.store(idle, Ordering::Relaxed);
    CURRENT_PROC.store(idle, Ordering::Relaxed);

    let shell = unsafe {
        let start = &raw const _binary_shell_bin_start;
        let size = (&raw const _binary_shell_bin_size).addr();
        core::slice::from_raw_parts(start, size)
    };
    create_process(shell);

    yield_now();
    panic!("switched to idle process");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(loc) => printf!("PANIC: {}:{}: {}\n", loc.file(), loc.line(), info.message()),
        None => printf!("PANIC: {}\n", info.message()),
    }
    loop {}
}
