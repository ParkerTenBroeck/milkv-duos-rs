./build_os.sh

cd deploy
cargo run -- /dev/ttyUSB0 ../kernel/target/milkv-duos/release/kernel.bin