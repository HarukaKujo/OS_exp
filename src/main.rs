#![no_std]
#![no_main]
#![feature(panic_info_message)]


mod lang_items;
mod sbi;
mod console;
mod logging;


core::arch::global_asm!(include_str!("entry.asm"));


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }

    clear_bss();
    logging::init();

    println!("[kernel] Hello, world!");
    log::trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    log::debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize,
        erodata as usize
    );
    log::info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize,
        edata as usize
    );
    log::warn!(
        "[kernel] boot_stack top={:#x}, bottom={:#x}",
        boot_stack_top as usize,
        boot_stack as usize
    );
    log::error!(
        "[kernel] .bss [{:#x}, {:#x})",
        sbss as usize,
        ebss as usize
    );

    sbi::shutdown()
}
