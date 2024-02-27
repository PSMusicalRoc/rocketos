#![no_std]
#![no_main]

// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]

mod vga_buffer;

use core::panic::PanicInfo;

use vga_buffer::*;


// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests...", tests.len());
//     for test in tests {
//         test();
//     }
// }



#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Printing hello world to the screen :)
    print!("Deez Nuts\n\nGottem");
    WRITER.lock().write_byte(0x01);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}