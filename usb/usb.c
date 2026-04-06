#include "usb.h"

void dbus_configurate_full_speed(
    GPIO_reg* dbus, 
    uint8_t num, 
    char letter,
    DBuses which_dbus
){
    gpio_clock_enable(num);
    gpio_alternate_mode_enable(dbus, num, 10);
    gpio_set_speed(dbus, num, GPIOSpeedThree);

    // set full speed
    // D+ pull-up
    // D- pull-down
    switch (which_dbus) {
    case DBUSPLUS:
        gpio_enable_pull_up(dbus, num);    
        break;
    default:
        gpio_enable_pull_down(dbus, num);
        break;
    } 
}

inline void dbus_do_handshake(void){

}

// 9.4 of USB 2.0 specification
void handle_default_request(
    USBSetupPacket packet,
    GPIO_reg* dplus,
    GPIO_reg* dminus
){
    if(
        packet.bmRequestType == 0x00000000B || 
        packet.bmRequestType == 0x00000001B ||
        packet.bmRequestType == 0x00000010B
    ) 
        clear_feature_handler(dplus, dminus, packet);
    else if(
        packet.bmRequestType == 0x10000000B &&
        packet.wLenth == 0
    )
        get_configuration_handler(dplus, dminus);
    else if(packet.bmRequestType == 0x10000000B)
        get_descrtiptor_handler(dplus, dminus, packet);
    else if(packet.bmRequestType == 0x10000001B)
        get_interface_handler(dplus, dminus, packet);
}