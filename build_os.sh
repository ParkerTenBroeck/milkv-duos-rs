cd os  || exit 1
cargo build --release  || exit 1
riscv64-none-elf-objcopy -O binary ./target/milkv-duos/release/os ./target/milkv-duos/release/os.bin  || exit 1
cd ..  || exit 1
