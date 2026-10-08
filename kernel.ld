ENTRY(boot)

SECTIONS {
    . = 0x80200000;

    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    . = ALIGN(4);
    . += 128 * 1024;
    __stack_top = .;
}
