
// #define uint32_t unsigned int

// #include "include/top_reg.h"
#include "include/platform_def.h"
#include "uart.c"

// Start of addition
#define UART_DLL 0x04140000
#define UART_DLH 0x04140004
#define UART_LCR 0x0414000C

void set_baudrate()
{
	// 14 for 115200, 13 for 128000
	int baud_divisor = 14;

	// set DLAB to 1 to set dll and dlh
	*(volatile uint32_t*)(UART_LCR) |= (uint32_t)0x80;

	// set divisor
	*(volatile uint32_t*)(UART_DLL) = (uint32_t)(baud_divisor & 0xff);
	*(volatile uint32_t*)(UART_DLH) = (uint32_t)((baud_divisor >> 8) & 0xff);

	// set DLAB back to 0
	*(volatile uint32_t*)(UART_LCR) &= (uint32_t)(~0x80);
}
// End of addition


void bl_early_platform_setup(){
    console_init(0, PLAT_UART_CLK_IN_HZ, 115200);
    while(1){
        console_puts("Hello, World!\n");
    }
}
