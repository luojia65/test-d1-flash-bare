PACKAGE_NAME := test-d1-flash-bt0
TARGET_DIR := target/riscv64imac-unknown-none-elf
DEBUG_DIR := $(TARGET_DIR)/debug
RELEASE_DIR := $(TARGET_DIR)/release
# flash part type on your board; can be 'spinor' or 'spinand' (used by `xfel`)
FLASH_TYPE := spinor
XFEL := xfel

all: release flashbin

flash: all
	echo "Writing image to flash"
	$(XFEL) $(FLASH_TYPE) write 0x0 $(RELEASE_DIR)/flash.bin

debug:
	cargo build -p $(PACKAGE_NAME)

release:
	cargo build -p $(PACKAGE_NAME) --release

objdump:
	rust-objdump $(DEBUG_DIR)/$(PACKAGE_NAME) -d

flashbin:
	rust-objcopy $(RELEASE_DIR)/$(PACKAGE_NAME) \
		--binary-architecture=riscv64 --strip-all -O binary $(RELEASE_DIR)/flash.bin

hexdump: all
	xxd $(RELEASE_DIR)/flash.bin

prepare:
	rustup target add riscv64imac-unknown-none-elf --toolchain nightly
	cargo +nightly install cargo-binutils
	rustup component add --toolchain nightly llvm-tools-preview
