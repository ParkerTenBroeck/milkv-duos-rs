mkdir -p out || exit 1

riscv64-none-elf-gcc \
	-DRISCV \
	-D__ASSEMBLY__ \
    -march=rv64imafdcv -mstrict-align \
	-mcmodel=medany \
	-mabi=lp64d \
	-ffreestanding  \
	-Wa,--fatal-warnings \
    -g -c -Wa,--gdwarf-2 \
    -nostdlib \
    -mno-plt  bl.S -o ./out/bl.S.o  || exit 1

riscv64-none-elf-gcc \
	-DRISCV \
	-D__ASSEMBLY__ \
    -march=rv64imafdcv -mstrict-align \
	-mcmodel=medany \
	-mabi=lp64d \
	-ffreestanding -fno-builtin -Wall -std=gnu99   \
	-Wa,--fatal-warnings \
    -g -c -Wa,--gdwarf-2 \
	-Os -ffunction-sections -fdata-sections \
	-fno-delete-null-pointer-checks \
    -mno-plt -fpie bl.c -o ./out/bl.c.o  || exit 1


riscv64-none-elf-ld -o bl -T bl.ld ./out/bl.S.o ./out/bl.c.o  || exit 1

riscv64-none-elf-objcopy -O binary bl bl.bin  || exit 1
