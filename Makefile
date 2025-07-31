# cat2verilog Makefile

.PHONY: build test clean run-example install uninstall help

# 預設目標
all: build

# 建置專案
build:
	@echo "Building cat2verilog..."
	cargo build

# 建置 release 版本
release:
	@echo "Building cat2verilog (release)..."
	cargo build --release

# 執行測試
test:
	@echo "Running tests..."
	cargo test

# 檢查程式碼
check:
	@echo "Checking code..."
	cargo check

# 清理建置檔案
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f *.v *.vcd *.log

# 執行範例
run-example: build
	@echo "Running example..."
	./target/debug/cat2verilog example.cat example.v
	@echo "Generated Verilog:"
	@cat example.v

# 安裝到系統
install: release
	@echo "Installing cat2verilog..."
	sudo cp target/release/cat2verilog /usr/local/bin/
	@echo "cat2verilog installed successfully!"

# 從系統移除
uninstall:
	@echo "Uninstalling cat2verilog..."
	sudo rm -f /usr/local/bin/cat2verilog
	@echo "cat2verilog uninstalled successfully!"

# 格式化程式碼
fmt:
	@echo "Formatting code..."
	cargo fmt

# 檢查程式碼風格
clippy:
	@echo "Running clippy..."
	cargo clippy

# 建立文件
doc:
	@echo "Generating documentation..."
	cargo doc --open

# 顯示幫助
help:
	@echo "cat2verilog Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build        - Build the project (debug)"
	@echo "  release      - Build the project (release)"
	@echo "  test         - Run tests"
	@echo "  check        - Check code without building"
	@echo "  clean        - Clean build artifacts"
	@echo "  run-example  - Run the example"
	@echo "  install      - Install to system"
	@echo "  uninstall    - Remove from system"
	@echo "  fmt          - Format code"
	@echo "  clippy       - Run clippy linter"
	@echo "  doc          - Generate documentation"
	@echo "  help         - Show this help" 