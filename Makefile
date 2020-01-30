ARCH ?= x86_64
TARGET ?= $(ARCH)-unknown-brutos-kernel

CFG ?= debug

ifeq ($(CFG),release)
CARGO_FLAGS += --release
endif

BUILD_DIR ?= target/$(TARGET)/$(CFG)

$(BUILD_DIR)/brutos-kernel: always-run kernel/src/arch/x86_64/page_tables.S
	# xargo rustc -p brutos-kernel --target $(TARGET) $(CARGO_FLAGS)
	xargo rustc -p brutos-kernel --target $(TARGET) $(CARGO_FLAGS) -- -C link-arg=-Tkernel/$(ARCH).lds -C link-arg=-n

kernel/src/arch/x86_64/page_tables.S: kernel/src/arch/x86_64/page_tables.py
	python3 $^ > $@

$(BUILD_DIR)/brutos-kernel.img: $(BUILD_DIR)/brutos-kernel
	rm -rf $(BUILD_DIR)/rabid_kernel.img $(BUILD_DIR)/stage
	mkdir -p $(BUILD_DIR)/stage
	cp $^ $(BUILD_DIR)/stage
	hdiutil create -fs fat32 -imagekey diskimage-class=CRawDiskImage -srcfolder $(BUILD_DIR)/stage $@
	mv $@.dmg $@


.PHONY: qemu
qemu: $(BUILD_DIR)/brutos-kernel.img
	qemu-system-x86_64 -hda assets/grub.iso -hdb $(BUILD_DIR)/brutos-kernel.img -cpu Icelake-Client-noTSX

.PHONY: bochs
bochs: $(BUILD_DIR)/brutos-kernel.img
	bochs -f assets/bochsrc.$(CFG)

.PHONY: always-run
always-run:

default: $(BUILD_DIR)/brutos-kernel
