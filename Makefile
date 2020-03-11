target := riscv32-os
mode := debug
kernel := target/$(target)/$(mode)/mini-os
bin := target/$(target)/$(mode)/kernel.bin

.PHONY: all clean run build asm qemu kernel

all: build

build: $(bin)

run: build qemu

kernel:
	@cargo xbuild --target riscv32-os.json

$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $@

asm:
	@riscv64-unknown-elf-objdump -d $(kernel) | less

qemu:
	@/home/qiulei/riscv/qemu/build/riscv32-softmmu/qemu-system-riscv32 -nographic -machine virt \
		-kernel opensbi/virt.elf \
		-device loader,file=$(bin),addr=0x80400000
