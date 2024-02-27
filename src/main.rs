#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu_tests;
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[allow(unused_imports)]
use qemu_tests::Testable;

#[warn(unused_imports)]


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    
    use qemu_tests::{
        exit_qemu,
        QemuExitCode
    };

    serial_println!("Running {} tests...", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}



#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Printing hello world to the screen :)
    println!("Welcome to RocketOS Version {}!", env!("CARGO_PKG_VERSION"));

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu_tests::exit_qemu;

    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(qemu_tests::QemuExitCode::Failed);
    loop {}
}



/* TESTS */

#[test_case]
fn dumbass_stupid_test() {
    for i in 0..10 {
        assert!(i == i);
    }
}
