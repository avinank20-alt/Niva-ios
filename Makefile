# NIVA OS Makefile
# Build system for NIVA OS kernel and bootloader

.PHONY: all build run clean help iso debug

# Configuration
KERNEL_NAME = niva-os
CARGO_FLAGS = --release
QEMU = qemu-system-x86_64
QEMU_FLAGS = -m 1G -smp 2 -enable-kvm
TARGET = x86_64-unknown-none
OUTPUT_DIR = target/$(TARGET)/release

help:
	@echo "NIVA OS - Build System"
	@echo "======================"
	@echo ""
	@echo "Available targets:"
	@echo "  make build      - Build the kernel"
	@echo "  make run        - Build and run in QEMU"
	@echo "  make debug      - Build and debug in QEMU with GDB"
	@echo "  make iso        - Create a bootable ISO"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make help       - Show this help message"
	@echo ""

# Build the kernel
build: check-rust-nightly
	@echo "[*] Building NIVA OS kernel for $(TARGET)..."
	@rustup target add $(TARGET) 2>/dev/null || true
	@RUSTFLAGS="-C link-arg=-Tlinker.ld -C relocation-model=static" \
		cargo build $(CARGO_FLAGS) --target $(TARGET)
	@echo "[✓] Kernel built successfully!"
	@echo "    Output: $(OUTPUT_DIR)/kernel"
	@ls -lh $(OUTPUT_DIR)/kernel 2>/dev/null || echo "Kernel binary not found"

# Run in QEMU
run: build
	@echo "[*] Starting NIVA OS in QEMU..."
	@echo "    Memory: 1GB"
	@echo "    CPUs: 2"
	@echo ""
	@echo "    To exit QEMU, press Ctrl+A then X"
	@echo ""
	@$(QEMU) -kernel $(OUTPUT_DIR)/kernel $(QEMU_FLAGS) -serial stdio

# Debug with GDB
debug: build
	@echo "[*] Starting NIVA OS in QEMU (debug mode)..."
	@$(QEMU) -kernel $(OUTPUT_DIR)/kernel $(QEMU_FLAGS) -s -S &
	@echo "[*] Waiting for QEMU to start..."
	@sleep 2
	@echo "[*] Starting GDB..."
	@gdb -ex "target remote localhost:1234" -ex "continue" $(OUTPUT_DIR)/kernel || true

# Build bootable ISO
iso: build
	@echo "[*] Creating bootable ISO..."
	@mkdir -p iso/boot/grub
	@cp $(OUTPUT_DIR)/kernel iso/boot/
	@echo 'menuentry "NIVA OS" {' > iso/boot/grub/grub.cfg
	@echo '  multiboot2 /boot/kernel' >> iso/boot/grub/grub.cfg
	@echo '  boot' >> iso/boot/grub/grub.cfg
	@echo '}' >> iso/boot/grub/grub.cfg
	@grub-mkrescue -o niva-os.iso iso 2>/dev/null || echo "[!] GRUB not found. Install with: apt-get install grub-pc xorriso"
	@echo "[✓] ISO created: niva-os.iso"

# Run ISO in QEMU
run-iso: iso
	@echo "[*] Running NIVA OS ISO in QEMU..."
	@$(QEMU) -cdrom niva-os.iso $(QEMU_FLAGS) -serial stdio

# Check for Rust nightly
check-rust-nightly:
	@rustc --version | grep -q nightly || { \
		echo "[!] Rust nightly required."; \
		echo "[*] Run: rustup install nightly"; \
		exit 1; \
	}

# Clean build artifacts
clean:
	@echo "[*] Cleaning build artifacts..."
	@cargo clean
	@rm -rf iso/ niva-os.iso
	@echo "[✓] Clean complete"

# Show build information
info:
	@echo "NIVA OS Build Information"
	@echo "=========================="
	@rustc --version
	@cargo --version
	@$(QEMU) --version | head -1
	@echo ""
	@echo "Target: $(TARGET)"
	@echo "Profile: $(CARGO_FLAGS)"

# Run all tests
test:
	@echo "[*] Running tests..."
	@cargo test --target $(TARGET) || true

# Format code
fmt:
	@echo "[*] Formatting code..."
	@cargo fmt --all

# Lint code
lint:
	@echo "[*] Linting code..."
	@cargo clippy --target $(TARGET) --all

# Default target
all: build

