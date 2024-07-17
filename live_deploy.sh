./build_os.sh

cd deploy
cargo run -- /dev/ttyUSB0 ../os/target/milkv-duos/release/os.bin