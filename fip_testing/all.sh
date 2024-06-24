cd bl  || exit 1
./build.sh  || exit 1
cd ..  || exit 1
cp ./bl/bl.bin ./make/bl2.bin  || exit 1
cargo run  || exit 1
cp fip.bin /run/media/may/boot/fip.bin  || exit 1