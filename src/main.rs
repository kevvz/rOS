#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;
use core::panic::PanicInfo;
// use ros::println;


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info:&PanicInfo) ->! {
    println!("{}",_info);
    ros::hlt_loop();
} 

#[no_mangle]
pub extern "C" fn _start() ->!{
    println!("bomba......");

    ros::init();

    // x86_64::instructions::interrupts::int3();
    
    // unsafe {
    //     *(0xdeadbeaf as *mut u8) = 42;
    // }
   
    
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    let ptr = 0x2031b2 as *mut u8;
    unsafe{*ptr = 42};  
  


    #[cfg(test)]
    test_main();

    println!("Woopee!");
    ros::hlt_loop();
}




