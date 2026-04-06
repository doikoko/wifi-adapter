#pragma once

#include "data.h"
#include "rcc.h"

#define GPIO_BASE       0x40020000

#define GPIOC_BASE   (GPIO_BASE + 0x800)
#define GPIOC        ((GPIO_reg*)GPIOC_BASE)

#define GPIOB_BASE   (GPIO_BASE + 0x400)
#define GPIOB        ((GPIO_reg*)GPIOB_BASE)


typedef struct {
    volatile uint32_t moder;
    volatile uint32_t otyper;
    volatile uint32_t ospeedr;
    volatile uint32_t pupdr;
    volatile uint32_t idr;
    volatile uint32_t odr;
    volatile uint32_t bsrr;
    volatile uint32_t lckr;
    volatile uint32_t afrl;
    volatile uint32_t afrh;
} GPIO_reg;

typedef enum { 
    GPIOSpeedZero, 
    GPIOSpeedOne, 
    GPIOSpeedTwo, 
    GPIOSpeedThree 
} GpioSpeed;

inline void gpio_set_output_mode(GPIO_reg* gpio, uint8_t num){
    gpio->moder &= ~(0x3 << (2 * num));
    gpio->moder |= 0b10 << (2 * num);
}

inline void gpio_enable_push_pull(GPIO_reg* gpio, uint8_t num){
    gpio->otyper &= ~(1 << num);
}

inline void gpio_set_speed(GPIO_reg* gpio, uint8_t num, GpioSpeed speed){
    gpio->ospeedr &= ~(0x3 << (2 * num));
    gpio->ospeedr |= ((uint8_t)speed) << (2 * num);
}   

inline void gpio_enable_pull_up(GPIO_reg* gpio, uint8_t num){
    gpio->pupdr &= ~(0x3 << (2 * num));
    gpio->pupdr |= 0b01 << (2 * num);
}

inline void gpio_enable_pull_down(GPIO_reg* gpio, uint8_t num){
    gpio->pupdr &= ~(0x3 << (2 * num));
    gpio->pupdr |= 0b10 << (2 * num);
}

inline void gpio_alternate_mode_enable(
    GPIO_reg* gpio,
    uint8_t num,
    uint8_t func_num
){
#define MAX_FUNC 16
    gpio->moder &= ~(0x3 << (2 * num));
    gpio->moder |= 0b10 << (2 * num);
    
    if (func_num >= MAX_FUNC) return;
    if (num < 8) gpio->afrl |= func_num << (num * 4);
    else if (num < MAX_FUNC) gpio->afrh |= func_num << ((num - 8) * 4);

}