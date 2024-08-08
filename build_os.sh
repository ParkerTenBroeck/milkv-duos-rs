cd kernel || exit 1
cargo build --release  || exit 1
riscv64-none-elf-objcopy -O binary ./target/milkv-duos/release/kernel ./target/milkv-duos/release/kernel.bin  || exit 1
cd ..  || exit 1
