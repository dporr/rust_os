use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end                              
        };
        tss
    };
}


use x86_64::structures::gdt::{ GlobalDescriptorTable, Descriptor};
lazy_static! {
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt: GlobalDescriptorTable = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_descriptor = Descriptor::tss_segment(&TSS);
        gdt.add_entry(tss_descriptor);
        gdt
    };

}

fn init(){
    GDT::load();
}