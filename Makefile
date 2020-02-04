ARCH ?= x86_64
TARGET ?= $(ARCH)-unknown-brutos-kernel

CFG ?= debug

ifeq ($(CFG),release)
CARGO_FLAGS += --release
else
RUSTFLAGS += -C force-frame-pointers=yes
endif

BUILD_DIR ?= target/$(TARGET)/$(CFG)

export RUSTFLAGS

$(BUILD_DIR)/brutos-kernel: always-run kernel/src/arch/x86_64/page_tables.S kernel/src/arch/x86_64/interrupt/entry.rs
	xargo rustc -p brutos-kernel --target $(TARGET) $(CARGO_FLAGS) -- -C link-arg=-Tkernel/$(ARCH).lds -C link-arg=-n

kernel/src/arch/x86_64/page_tables.S: kernel/src/arch/x86_64/page_tables.py
	python3 $^ > $@

kernel/src/arch/x86_64/interrupt/entry.rs: kernel/src/arch/x86_64/interrupt/entry.py
	python3 $^ > $@


$(BUILD_DIR)/brutos-kernel.iso: $(BUILD_DIR)/brutos-kernel
	rm -f $@
	cd $(BUILD_DIR); xorriso -outdev brutos-kernel.iso -add brutos-kernel


.PHONY: qemu
qemu: $(BUILD_DIR)/brutos-kernel.iso
	qemu-system-x86_64 -drive if=ide,index=0,format=raw,file=assets/grub.iso -drive if=ide,index=1,format=raw,file=$(BUILD_DIR)/brutos-kernel.iso -cpu qemu64,+fsgsbase

.PHONY: bochs
bochs: $(BUILD_DIR)/brutos-kernel.iso
	bochs -f assets/bochsrc.$(CFG)


.PHONY: always-run
always-run:

default: $(BUILD_DIR)/brutos-kernel
