#![no_std]
#![no_main]
use core::panic::PanicInfo;


#[panic_handler]
// PanicInfo parameter contains the file and line
// where the panic happened and the optional panic message

// -> ! indicates a diverging function
// (A function that do not retur)
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//invoked directly by the operating system or bootloader.
pub extern "C" fn _start() -> ! {
    loop {}
}