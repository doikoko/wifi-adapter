#pragma once

#include "../drivers/data.h"
#include "../drivers/gpio.h"

typedef struct {
// if bit is set - Host to Device
// else Device to Host
#define SETUP_PACKET_DIRECTION_MASK     0b10000000

// 0 - standard
// 1 - class
// 2 - vendor
// 3 - reserved
#define SETUP_PACKET_TYPE_MASK      0b01100000

// 0 - device
// 1 - interface
// 2 - endpoint
// 3 - other
// 4-31 - reserved
#define RECIPIENT_MASK      0b11111
    uint8_t bmRequestType;

    uint8_t bRequest;
    uint16_t wValue;
    uint16_t wIndex;
    uint16_t wLenth;
} USBSetupPacket;

typedef struct {
    uint32_t SYNC;

#define PID_POCKET_TYPE_MASK    0b1111000
#define PID_POCKET_CHECK_MASK   0b00001111
    uint8_t PID;


} USBPacketFullSpeed;

typedef enum { DBUSPLUS, DBUSMINUS } DBuses;
void dbus_configurate_full_speed(
    GPIO_reg* dbus, 
    uint8_t num, 
    char letter,
    DBuses which_dbus
);

inline void dbus_do_handshake(void);

// 9.4 of USB 2.0 specification
void handle_default_request(
    USBSetupPacket packet,
    GPIO_reg* dplus,
    GPIO_reg* dminus
);

void clear_feature_handler(
    GPIO_reg* dplus,
    GPIO_reg* dminus,
    USBSetupPacket packet
);

void get_configuration_handler(
    GPIO_reg* dplus,
    GPIO_reg* dminus
);

void get_descrtiptor_handler(
    GPIO_reg* dplus,
    GPIO_reg* dminus,
    USBSetupPacket packet
);

void get_interface_handler(
    GPIO_reg* dplus,
    GPIO_reg* dminus
)