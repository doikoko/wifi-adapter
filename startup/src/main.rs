#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
#![no_main]

use main::main;

const SRAM_START: u32 = 0x20000000;
const SRAM_SIZE:  u32 = 64 * 1024; // 64K
const SRAM_END:   u32 = SRAM_START + SRAM_SIZE;
const STACK_POINTER_FIRST_ADDR: u32 = SRAM_END;

const VECTOR_TABLE_SIZE_WORDS: usize = 255;

unsafe extern "C" fn default_handler(){
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

macro_rules! num_to_ptr {
    ($num: expr) => {
        $num as *const () as *const unsafe extern "C" fn()
    };
}
#[unsafe(link_section = ".isr_vector")]
#[unsafe(no_mangle)]
pub static mut ISR_VECTOR: [*const unsafe extern "C" fn(); VECTOR_TABLE_SIZE_WORDS] = {
    const NULL: *const unsafe extern "C" fn() = num_to_ptr!(0);
    
    let mut table = [NULL; VECTOR_TABLE_SIZE_WORDS];
    
    table[0] = num_to_ptr!(STACK_POINTER_FIRST_ADDR);  // initial stack pointer
    table[1] = num_to_ptr!(reset_handler);
    table[2] = num_to_ptr!(default_handler);
    table[3] = num_to_ptr!(default_handler);
    table[4] = num_to_ptr!(default_handler);
    table[5] = num_to_ptr!(default_handler);
    table[6] = num_to_ptr!(default_handler);
    table[7] = NULL;
    table[8] = NULL;
    table[9] = NULL;
    table[10] = NULL;
    table[11] = num_to_ptr!(default_handler);
    table[12] = NULL;
    table[13] = NULL;
    table[14] = num_to_ptr!(default_handler);
    table[15] = num_to_ptr!(default_handler);

    table
};

unsafe extern "C" {
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
    static _etext: u32;

    static mut _srecieved_data: u32;
    static mut _erecieved_data: u32;
}
unsafe extern "C" fn reset_handler() {
    let sdata = &raw mut _sdata;
    let edata = &raw mut _edata;
    let sbss  = &raw mut _sbss;
    let ebss  = &raw mut _ebss;
    let etext = &raw const _etext;

    let srecieved_data = &raw mut _srecieved_data;
    let erecieved_data = &raw mut _erecieved_data;

    let data_size = (edata as usize) - (sdata as usize);
    let bss_size  = (ebss  as usize) - (sbss  as usize);
    let recieved_data_size =
        (erecieved_data as usize) - (srecieved_data as usize);

    // zero .bss
    for i in 0..(bss_size / core::mem::size_of::<u32>()) {
        sbss.add(i).write_volatile(0);
    }

    // copy .data from FLASH to SRAM
    for i in 0..(data_size / core::mem::size_of::<u32>()) {
        let val = etext.add(i).read_volatile();
        sdata.add(i).write_volatile(val);
    }

    // clear received_data
    for i in 0..(recieved_data_size / core::mem::size_of::<u32>()) {
        srecieved_data.add(i).write_volatile(0);
    }

    main();
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> !{
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}