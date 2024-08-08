cd bl  || exit 1
cargo build --release  || exit 1
riscv64-none-elf-objcopy -O binary ./target/milkv-duos/release/bl ./target/milkv-duos/release/bl.bin  || exit 1
cd ..  || exit 1

#cd os  || exit 1
#cargo build --release  || exit 1
#riscv64-none-elf-objcopy -O binary ./target/milkv-duos/release/os ./target/milkv-duos/release/os.bin  || exit 1
#cd ..  || exit 1

cd fip || exit 1
cargo run -- -bl ../bl/target/milkv-duos/release/bl.bin -o ../fip.bin || exit 1
cd ..
# change this to where the fip.bin needs to be on the boot SD card
cp fip.bin /run/media/may/boot/fip.bin  || exit 1