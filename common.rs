use core::fmt::{self, Write};

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            crate::putchar(ch);
        }
        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    let _ = Console.write_fmt(args);
}

macro_rules! printf {
    ($($arg:tt)*) => {
        $crate::common::print_fmt(format_args!($($arg)*))
    };
}

pub const PAGE_SIZE: usize = 4096;

pub const SYS_PUTCHAR: usize = 1;
pub const SYS_GETCHAR: usize = 2;
pub const SYS_EXIT: usize = 3;

pub type PAddr = usize;
pub type VAddr = usize;

pub fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

pub fn is_aligned(value: usize, align: usize) -> bool {
    value & (align - 1) == 0
}
