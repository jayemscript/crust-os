# Documentation-only placeholder. No build or execution targets exist.
.DEFAULT_GOAL := help
.PHONY: help

help:
	@echo "crust-os is in the planning stage."
	@echo "Read README.md, ROADMAP.md, and docs/README.md."

ASM = nasm
QEMU = qemu-system-x86_64

all: boot

boot:
	$(ASM) -f bin kernel/boot.asm -o boot.bin

run: boot
	$(QEMU) -drive format=raw,file=boot.bin

clean:
	rm -f boot.bin

check_main:
	rustc kernel/main.rs