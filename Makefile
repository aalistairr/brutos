ARCH ?= x86_64
TARGET ?= $(ARCH)-unknown-brutos-kernel
VMWARE_VMRUN ?= /Applications/VMware Fusion.app/Contents/Library/vmrun
VMWARE_VMS_DIR ?= $(HOME)/Virtual Machines.localized

CFG ?= debug

VMWARE_VMX ?= $(VMWARE_VMS_DIR)/BrutOS ($(CFG)).vmwarevm/BrutOS ($(CFG)).vmx

ifeq ($(CFG),release)
CARGO_FLAGS += --release
CARGO_FLAGS_RUSTFLAGS += -C lto
else
TARGET := $(TARGET)-debug
RUSTFLAGS += -C force-frame-pointers=yes
endif

BUILD_DIR ?= target/$(TARGET)/$(CFG)

export RUSTFLAGS

RUST_TARGET_PATH = $(shell pwd)
export RUST_TARGET_PATH

$(BUILD_DIR)/brutos-kernel: always-run kernel/src/arch/x86_64/page_tables.S kernel/src/arch/x86_64/interrupt/entry.rs
	xargo rustc -p brutos-kernel --target $(TARGET) $(CARGO_FLAGS) -- -C link-arg=-Tkernel/$(ARCH).lds -C link-arg=-n $(CARGO_FLAGS_RUSTFLAGS)

kernel/src/arch/x86_64/page_tables.S: kernel/src/arch/x86_64/page_tables.py
	python3 $^ > $@

kernel/src/arch/x86_64/interrupt/entry.rs: kernel/src/arch/x86_64/interrupt/entry.py
	python3 $^ > $@


$(BUILD_DIR)/brutos-init: always-run
	xargo rustc -p brutos-init --target $(TARGET) $(CARGO_FLAGS) -- $(CARGO_FLAGS_RUSTFLAGS)

$(BUILD_DIR)/brutos-init.cpio: $(BUILD_DIR)/brutos-init
	rm -f $@
	cd $(BUILD_DIR); echo brutos-init | cpio -o > brutos-init.cpio

$(BUILD_DIR)/brutos-kernel.iso: $(BUILD_DIR)/brutos-kernel $(BUILD_DIR)/brutos-init.cpio
	rm -f $@
	cd $(BUILD_DIR); xorriso -outdev brutos-kernel.iso -add brutos-kernel brutos-init.cpio

$(BUILD_DIR)/brutos-kernel.vmdk: $(BUILD_DIR)/brutos-kernel.iso
	rm -f $@
	qemu-img convert -f raw -O vmdk $< $@

.PHONY: iso
iso: $(BUILD_DIR)/brutos-kernel.iso
.PHONY: vmdk
vmdk: $(BUILD_DIR)/brutos-kernel.vmdk


.PHONY: qemu
qemu: $(BUILD_DIR)/brutos-kernel.iso
	qemu-system-x86_64 -drive if=ide,index=0,format=raw,file=assets/grub.iso -drive if=ide,index=1,format=raw,file=$(BUILD_DIR)/brutos-kernel.iso -cpu qemu64,+fsgsbase $(QEMUFLAGS)

.PHONY: bochs
bochs: $(BUILD_DIR)/brutos-kernel.iso
	bochs -f assets/bochsrc.$(CFG)

.PHONY: vmware
vmware: $(BUILD_DIR)/brutos-kernel.vmdk
	"$(VMWARE_VMRUN)" list | grep "$(VMWARE_VMX)" && "$(VMWARE_VMRUN)" stop "$(VMWARE_VMX)" hard || true
	"$(VMWARE_VMRUN)" start "$(VMWARE_VMX)"

.PHONY: always-run
always-run:

default: $(BUILD_DIR)/brutos-kernel
