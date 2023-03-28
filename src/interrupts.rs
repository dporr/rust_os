use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

use lazy_static::lazy_static;
lazy_static! {
    static ref IDT: InterruptDescriptorTable =  {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_exception);
        idt
    };
}
pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_exception(
        stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);  
}