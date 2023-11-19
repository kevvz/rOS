#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::{QemuExitCode,exit_qemu,serial_println};
use ros::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Running {} tests",tests.len());
    for test in tests {
        test();
        serial_println!("[no panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
} 

#[panic_handler]
fn panic(info:&PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
} 

#[test_case]
fn should_fail()     {
    serial_print!("should_panic::should fail...\t");
    assert_eq!(0,1);
}