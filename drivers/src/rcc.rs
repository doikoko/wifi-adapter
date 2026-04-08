#![allow(dead_code)]

use core::cell::LazyCell;
use vcell::VolatileCell;

const RCC_BASE: u32 = 0x40023800;

#[repr(C)]
pub struct RCC{
    pub cr: VolatileCell<u32>,
    pub pllcfgr: VolatileCell<u32>,
    pub cfgr: VolatileCell<u32>,
    pub cir: VolatileCell<u32>,
    pub ahb1rstr: VolatileCell<u32>,
    pub ahb2rstr: VolatileCell<u32>,
    pub reserved0: VolatileCell<[u32; 2]>,
    pub apb1rstr: VolatileCell<u32>,
    pub apb2rstr: VolatileCell<u32>,
    pub reserved1: VolatileCell<[u32; 2]>,
    pub ahb1enr: VolatileCell<u32>,
    pub ahb2enr: VolatileCell<u32>,
    pub reserved2: VolatileCell<[u32; 2]>,
    pub apb1enr: VolatileCell<u32>,
    pub apb2enr: VolatileCell<u32>,
    pub reserved3: VolatileCell<[u32; 2]>,
    pub rcc_ahb1lpenr: VolatileCell<u32>,
    pub rcc_ahb2lpenr: VolatileCell<u32>,
    pub reserved4: VolatileCell<[u32; 2]>,
    pub rcc_apb1lpenr: VolatileCell<u32>,
    pub rcc_apb2lpenr: VolatileCell<u32>,
    pub reserved5: VolatileCell<[u32; 2]>,
    pub rcc_bdcr: VolatileCell<u32>,
    pub rcc_csr: VolatileCell<u32>,
    pub reserved6: VolatileCell<[u32; 2]>,
    pub rcc_sscgr: VolatileCell<u32>,
    pub rcc_plli2scfgr: VolatileCell<u32>,
    pub reserved: VolatileCell<u32>,
    pub rcc_dckcfgr: VolatileCell<u32>,
}

impl<'a> RCC{
    pub fn new() -> &'a mut Self{
        unsafe{ &mut *(RCC_BASE as *mut Self) }
    }

    pub fn gpio_clock_enable(&mut self, letter: u8){
        if letter >= 'A' as u8 && letter <= 'E' as u8{
            self.ahb1enr.set(self.ahb1enr.get() | 1 << (letter - 'A' as u8));
        }
        else if letter == 'H' as u8{
            self.ahb1enr.set(self.ahb1enr.get() | 1 << 7);
        }
    }
}

pub static mut RCC_INST: LazyCell<&'static mut RCC> = LazyCell::new(|| RCC::new());