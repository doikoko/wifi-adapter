#pragma once

#include "data.h"

#define RCC_BASE    0x40023800
#define RCC ((RCC_reg*)RCC_BASE)

typedef struct {
    volatile uint32_t cr;
    volatile uint32_t pllcfgr;
    volatile uint32_t cfgr;
    volatile uint32_t cir;
    volatile uint32_t ahb1rstr;
    volatile uint32_t ahb2rstr;
    volatile uint32_t reserved0[2];
    volatile uint32_t apb1rstr;
    volatile uint32_t apb2rstr;
    volatile uint32_t reserved1[2];
    volatile uint32_t ahb1enr;
    volatile uint32_t ahb2enr;
    volatile uint32_t reserved2[2];
    volatile uint32_t apb1enr;
    volatile uint32_t apb2enr;
    volatile uint32_t reserved3[2];
    volatile uint32_t rcc_ahb1lpenr;
    volatile uint32_t rcc_ahb2lpenr;
    volatile uint32_t reserved4[2];
    volatile uint32_t rcc_apb1lpenr;
    volatile uint32_t rcc_apb2lpenr;
    volatile uint32_t reserved5[2];
    volatile uint32_t rcc_bdcr;
    volatile uint32_t rcc_csr;
    volatile uint32_t reserved6[2];
    volatile uint32_t rcc_sscgr;
    volatile uint32_t rcc_plli2scfgr;
    volatile uint32_t reserved;
    volatile uint32_t rcc_dckcfgr;
} RCC_reg;

inline void gpio_clock_enable(char letter){
    if(letter >= 'A' && letter <= 'E')
        RCC->ahb1enr |= 1 << (letter - 'A');
    else if (letter == 'H')    
        RCC->ahb1enr |= 1 << 7;
}