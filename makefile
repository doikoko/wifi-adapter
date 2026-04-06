CC = arm-none-eabi-gcc
OBJCOPY = arm-none-eabi-objcopy
LD = arm-none-eabi-ld
DRIVERS = ./drivers
COMPILE_FLAGS = -mcpu=cortex-m4 \
	-mthumb -O3 \
    -Wall -Wextra -mfloat-abi=hard -mfpu=fpv4-sp-d16 \
	-Wno-char-subscripts -Wno-main
LINK_FLAGS = -T mem.ld

all: blink.bin
blink.bin: blink.elf
	$(OBJCOPY) -O binary out_dir/blink.elf blink.bin

blink.elf: out_dir
	$(CC) $(COMPILE_FLAGS) src/main.c -c -o out_dir/main.o 
	$(CC) $(COMPILE_FLAGS) startup/startup.c -c -o out_dir/startup.o \

	$(LD) $(LINK_FLAGS) out_dir/main.o out_dir/startup.o -o out_dir/blink.elf

out_dir:
	mkdir out_dir


test:
	dfu-util -a 0 -D blink.bin --dfuse-address 0x08000000
clean:
	rm -rf blink.bin blink.elf out_dir ./pc.elf ./pc/assets/user.bin ./pc/assets/user.o ./pc/assets/user.elf
c_flash:
	dfu-util -a 0 -e