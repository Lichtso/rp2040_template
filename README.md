# rp2040 Project Template in Rust
Currently only supports the RP Pico board.

### Dev Dependencies (Which are pre-built)
```bash
arm-none-eabi-gcc -c -I device/bootloader -o device/bootloader/boot2_w25q080.o device/bootloader/boot2_w25q080.S
arm-none-eabi-gcc -c -I device/bootloader -o device/bootloader/start.o device/bootloader/start.S
```

### Dev Dependencies (You have to install / build yourself)
```bash
rustup target add thumbv6m-none-eabi
cargo build --release --manifest-path uf2tool/Cargo.toml
```

### Build and Upload
```bash
cd device/
cargo run --release
```

### Analysis (Disassembler)
```bash
objdump -disassemble -arch-name=thumb device/target/thumbv6m-none-eabi/release/rp2040_template
```