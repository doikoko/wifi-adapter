STARTUP_ELF = ./target/thumbv7*/release/startup

build:
	cargo build -p startup --release
	rust-objcopy -O binary $(STARTUP_ELF) blink.bin

test:
	dfu-util -a 0 -D blink.bin --dfuse-address 0x08000000