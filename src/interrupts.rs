use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

use lazy_static::lazy_static;
lazy_static! {
    static   ref IDT: InterruptDescriptorTable =  {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_exception);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}
pub fn init_idt() {
    IDT.load();
}


//TODO: Do all this by hand following:
// https://os.phil-opp.com/edition-1/extra/naked-exceptions/
extern "x86-interrupt" fn breakpoint_exception(
        stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);  
}

extern "x86-interrupt" fn double_fault_handler(
     stack_frame: InterruptStackFrame,
    _error_code: u64) -> !
{
    panic! ("EXCEPTION: #DF\n{:#?}", stack_frame);
}
