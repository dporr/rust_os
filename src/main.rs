#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
// PanicInfo parameter contains the file and line
// where the panic happened and the optional panic message

// -> ! indicates a diverging function
// (A function that do not retur)
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//invoked directly by the operating system or bootloader.
// Interesting way of controling compiler flags:
//cargo rustc -- -C link-arg=-nostartfiles
static HELLO: &[u8] = b"Hello HackTheBox GT!";
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
    vga_buffer::print_someting();
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