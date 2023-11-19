#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use ros::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info);
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

#[test_case]
fn test_println() {
    println!("testing output...");
}


// fn test_runner(tests: &[&dyn Fn()]) {
//     unimplemented!();
// }