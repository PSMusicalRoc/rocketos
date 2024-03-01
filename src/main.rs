#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rocket_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rocket_os::println;

#[allow(unused_imports)]

#[warn(unused_imports)]


#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Printing hello world to the screen :)
    println!("Welcome to RocketOS Version {}!", env!("CARGO_PKG_VERSION"));

    rocket_os::init();
    
    #[cfg(test)]
    test_main();

    rocket_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rocket_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rocket_os::test_panic_handler(info)
}
