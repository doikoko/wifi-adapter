#include "../drivers/gpio.h"
#include "../drivers/rcc.h"
#include "../usb/usb.h"

// check pins.md file
#define DPLUS           GPIOB
#define DPLUS_LETTER    'B'
#define DPLUS_NUM       6

// check pins.md file
#define DMINUS            GPIOB
#define DMINUS_LETTER     'B'
#define DMINUS_NUM        3


void main(void){
    dbus_configurate(DPLUS, DPLUS_NUM, DPLUS_LETTER, DBUSPLUS);
    dbus_configurate(DPLUS, DPLUS_NUM, DPLUS_LETTER, DBUSMINUS);
}