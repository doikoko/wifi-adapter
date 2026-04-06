#include "../drivers/data.h"
#include "../drivers/rcc.h"
#include "../drivers/gpio.h"

#define SRAM_START                  0x20000000U
#define SRAM_SIZE                   64U * 1024U //64K
#define SRAM_END                    SRAM_START + SRAM_SIZE
#define STACK_POINTER_FIRST_ADDR    ((uint32_t)SRAM_END)

#define VECTOR_TABLE_SIZE_WORDS     255

void reset_handler(void);
void nmi_handler(void)  __attribute((weak, alias("default_handler")));
void hard_fault_handler(void)   __attribute((weak, alias("default_handler")));
void memory_management_fault_handler(void)  __attribute((weak, alias("default_handler")));
void bus_fault_handler(void)    __attribute((weak, alias("default_handler")));
void usage_fault_handler(void)  __attribute((weak, alias("default_handler")));
void svcall_handler(void)   __attribute((weak, alias("default_handler")));
void pend_sv_handler(void)  __attribute((weak, alias("default_handler")));
void systick_handler(void)  __attribute((weak, alias("default_handler")));

void default_handler(){ while(1){ asm("wfi"); } }

void main(void);

extern uint32_t _sdata, _edata, _sbss, _ebss, _etext;

static volatile uint32_t isr_vector[VECTOR_TABLE_SIZE_WORDS]
__attribute__((section(".isr_vector"))) = {
    STACK_POINTER_FIRST_ADDR,             
    (uint32_t)&reset_handler,
    (uint32_t)&nmi_handler,
    (uint32_t)&hard_fault_handler,
    (uint32_t)&memory_management_fault_handler,
    (uint32_t)&bus_fault_handler,
    (uint32_t)&usage_fault_handler,
    0,0,0,0,
    (uint32_t)&svcall_handler,
    0,0,
    (uint32_t)&pend_sv_handler,
    (uint32_t)&systick_handler,
};

// entry point
void reset_handler(void){
    // zero bss
    volatile uint32_t* sdata = &_sdata; 
    volatile uint32_t* edata = &_edata; 
    volatile uint32_t* sbss = &_sbss; 
    volatile uint32_t* ebss = &_ebss;
    volatile uint32_t* etext = &_etext;
    
    uint32_t data_size = (uint32_t)(edata) - (uint32_t)(sdata);
    uint32_t bss_size = (uint32_t)(ebss) - (uint32_t)(sbss);
    
    for(uint32_t i = 0; i < bss_size / sizeof(uint32_t); i++){
        *(sbss + i) = 0;
    }
    
    // copy data from FLASH to SRAM
    for(uint32_t i = 0; i < data_size / sizeof(uint32_t); i++){
        *(sdata + i) = *(etext + i);
    }

    main();
}