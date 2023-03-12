#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
// PanicInfo parameter contains the file and line
// where the panic happened and the optional panic message

// -> ! indicates a diverging function
// (A function that do not retur)
fn panic(_info: &PanicInfo) -> ! {
    println! ("{}", _info);
    loop {}
}


//invoked directly by the operating system or bootloader.
// Interesting way of controling compiler flags:
//cargo rustc -- -C link-arg=-nostartfiles
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    //0xb8000 magic number indicating the location of the VGA buffer in memory
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte; // Set buffer[i * 2] to the char
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //Set background to light cyan 0xb
    //     }
    // }
   //vga_buffer::print_someting();
   //use core::fmt::Write;
   println! ("Using lazy loaded statics is awesome. \n\n\n");
   println! ("Some numbers: {} {}", 42, 666); 
   #[cfg(test)]
   test_main();
   loop {}
}

/*
CTF IDEA:
Since we can’t link to the C library of the operating system,
we need an alternative way to provide these functions to the
compiler. One possible approach for this could be to 
implement our own memset etc. functions and apply the
#[no_mangle] attribute to them (to avoid the automatic
renaming during compilation). However, this is dangerous
since the slightest mistake in the implementation of these
functions could lead to undefined behavior. 
*/


#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        //Magic number is the io port we declared in Cargo.toml
        let mut port =  Port::new(0xf4);
        port.write(exit_code as u32);
    }
}