#![no_std]
#![feature(alloc_error_handler)]

use alloc::string::ToString;
use static_alloc::Bump;

// initialize allocator
extern crate alloc;
#[global_allocator]
static A: Bump<[u8; 1 << 16]> = Bump::uninit();

// import function for printing
#[link(wasm_import_module = "arduino")]
extern {
    fn log_char(c: u8);
}
pub fn println(message: &str) {
    message.as_bytes().iter().for_each(|c| {
        unsafe { log_char(*c) };
    });

    unsafe { log_char('\n' as u8) };
}

// main function
#[no_mangle]
pub extern fn _start() {
    let mut i = u32::MAX;
    loop {
        i -= 1;
        // only allocation, should be cleaned up at end of loop
        let i_string = i.to_string();
        println(&i_string);
    }
}

// error handlers
#[panic_handler]
pub fn panic(_: &::core::panic::PanicInfo) -> ! {
    println("panic");
    core::arch::wasm32::unreachable();
}
#[alloc_error_handler]
pub fn oom(_: ::core::alloc::Layout) -> ! {
    println("out of memory");
    core::arch::wasm32::unreachable();
}
