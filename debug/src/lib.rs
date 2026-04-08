#![no_std]
#![allow(static_mut_refs)]

use drivers::{GPIO, RCC_INST, GPIOSpeed};

const LED_LETTER: u8 = 'C' as u8;
const LED_NUM:    u8 = 13;

pub struct LED<'a>{
    gpio: &'a mut GPIO,
    letter: u8,
    num: u8
}

impl<'a> LED<'a>{
    pub fn new() -> Self{
        Self{
            gpio: GPIO::new(LED_LETTER),
            letter: LED_LETTER,
            num: LED_NUM
        }
    }

    pub fn ebable_light(&mut self){
        self.gpio.odr.set(self.gpio.odr.get() & !(1 << self.num));
    }
    
    pub fn disable_light(&mut self){
        self.gpio.odr.set(self.gpio.odr.get() | 1 << self.num);
    }

    pub fn blink(&mut self){
        self.gpio.odr.set(self.gpio.odr.get() ^ 1 << self.num);
    }

    pub fn configure(&mut self){
        unsafe { RCC_INST.gpio_clock_enable(self.letter); };
        self.gpio.set_output_mode(self.num);
        self.gpio.enable_push_pull(self.num);
        self.gpio.set_speed(self.num, GPIOSpeed::Three);
    }
}