#![no_std]
#![no_main]

#[macro_use]
mod common;
mod user;

use user::{exit, getchar, putchar, readfile, writefile};

#[unsafe(no_mangle)]
extern "C" fn main() {
    'prompt: loop {
        printf!("> ");
        let mut cmdline = [0u8; 128];
        let mut len = 0;
        loop {
            let ch = getchar() as u8;
            putchar(ch);
            if len == cmdline.len() - 1 {
                printf!("command line too long\n");
                continue 'prompt;
            } else if ch == b'\r' {
                printf!("\n");
                break;
            } else {
                cmdline[len] = ch;
                len += 1;
            }
        }

        let cmd = &cmdline[..len];
        if cmd == b"hello" {
            printf!("Hello world from shell!\n");
        } else if cmd == b"readfile" {
            let mut buf = [0u8; 128];
            let len = readfile(c"hello.txt", &mut buf);
            if len >= 0 {
                printf!("{}\n", core::str::from_utf8(&buf[..len as usize]).unwrap_or("?"));
            }
        } else if cmd == b"writefile" {
            writefile(c"hello.txt", b"Hello from shell!\n");
        } else if cmd == b"exit" {
            exit();
        } else {
            printf!("unknown command: {}\n", core::str::from_utf8(cmd).unwrap_or("?"));
        }
    }
}
