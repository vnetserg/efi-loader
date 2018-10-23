EFIINC          = /usr/include/efi
EFIINCS         = -I$(EFIINC) -I$(EFIINC)/x86_64 -I$(EFIINC)/protocol
LIB             = /usr/lib
EFI_CRT_OBJS    = $(LIB)/crt0-efi-x86_64.o
EFI_LDS         = $(LIB)/elf_x86_64_efi.lds

RUST_TARGET		= release
RUST_RELEASE 	= target/${RUST_TARGET}
RUSTC_FLAGS		= -C no-redzone

LDFLAGS         = -nostdlib -znocombreloc -T $(EFI_LDS) -shared \
		  -Bsymbolic -L $(LIB) $(EFI_CRT_OBJS) 

SRC				= src
IMG				= img
BIN 			= bin
BUILD			= build


all: dirs $(BIN)/loader.efi

dirs:
	mkdir -p $(BIN) $(BUILD)

clobber: clean

clean:
	rm -rf $(BUILD) $(BIN) $(IMG)/fat.img target

.PHONY: $(BUILD)/loader.a
$(BUILD)/loader.a:
	cargo rustc --${RUST_TARGET} -- $(RUSTC_FLAGS)
	cp $(RUST_RELEASE)/libefi_loader.a $@

$(BUILD)/loader.so: $(BUILD)/loader.a
	ld $(LDFLAGS) $^ -o $@ -lefi -lgnuefi

$(BIN)/loader.efi: $(BUILD)/loader.so
	objcopy -j .text -j .sdata -j .data -j .dynamic \
		-j .dynsym  -j .rel -j .rela -j .reloc \
		--target=efi-app-x86_64 $^ $@

$(IMG)/fat.img: $(BIN)/loader.efi
	dd if=/dev/zero of=$@ bs=1k count=1440
	mformat -i $@ -f 1440 ::
	mmd -i $@ ::/EFI
	mmd -i $@ ::/EFI/BOOT
	mcopy -i $@ $^ ::/EFI/BOOT/BOOTX64.EFI

qemu: $(IMG)/fat.img
	qemu-system-x86_64 -bios $(IMG)/OVMF.fd -drive file=$^,format=raw -nographic
