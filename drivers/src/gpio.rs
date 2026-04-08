use vcell::VolatileCell;

const GPIO_BASE: u32 = 0x40020000;
const MAX_FUNC: u8  = 16;

pub enum GPIOSpeed{ Zero, One, Two, Three }
#[repr(C)]
pub struct GPIO{
    pub moder: VolatileCell<u32>,
    pub otyper: VolatileCell<u32>,
    pub ospeedr: VolatileCell<u32>,
    pub pupdr: VolatileCell<u32>,
    pub idr: VolatileCell<u32>,
    pub odr: VolatileCell<u32>,
    pub bsrr: VolatileCell<u32>,
    pub lckr: VolatileCell<u32>,
    pub afrl: VolatileCell<u32>,
    pub afrh: VolatileCell<u32>,
}

impl<'a> GPIO{
    pub fn new(letter: u8) -> &'a mut Self{
        unsafe {
            &mut *((GPIO_BASE + (0x400 * (letter as u32 - ('A' as u32)))) as *mut Self)
        }
    }

    pub fn set_output_mode(&mut self, num: u8){
        self.moder.set(self.moder.get() & !(0x3 << (2 * num)));
        self.moder.set(self.moder.get() | 0b10 << (2 * num));
    }

    pub fn enable_push_pull(&mut self, num: u8){
        self.otyper.set(self.otyper.get() & !(1 << num));
    }

    pub fn set_speed(&mut self, num: u8, speed: GPIOSpeed){
        self.ospeedr.set(self.ospeedr.get() & !(0x3 << (2 * num)));
        self.ospeedr.set(self.ospeedr.get() | (speed as u32) << (2 * num));
    }   

    pub fn enable_pull_up(&mut self, num: u8){
        self.pupdr.set(self.pupdr.get() & !(0x3 << (2 * num)));
        self.pupdr.set(self.pupdr.get() | 0b01 << (2 * num));
    }

    pub fn enable_pull_down(&mut self, num: u8){
        self.pupdr.set(self.pupdr.get() & !(0x3 << (2 * num)));
        self.pupdr.set(self.pupdr.get() | 0b10 << (2 * num));
    }

    pub fn alternate_mode_enable(&mut self, num: u8, func_num: u8){
        self.moder.set(self.moder.get() & !(0x3 << (2 * num)));
        self.moder.set(self.moder.get() | 0b10 << (2 * num));
        
        if func_num < MAX_FUNC {
            if num < 8 {
                self.afrl.set(self.afrl.get() | (func_num as u32) << (num as u32) * 4);
            } else if num < MAX_FUNC {
                self.afrh.set(self.afrh.get() | (func_num as u32) << (((num as u32) - 8) * 4));
            } 
        }

    }
}