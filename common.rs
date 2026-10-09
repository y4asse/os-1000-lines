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
