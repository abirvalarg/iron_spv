#include "type.h"

extern byte _STACK, _DATA_START, _DATA_START_FLASH, _DATA_END, _BSS_START, _BSS_END;
void _reset();
void _NMI();
void _hardfault();
void _SVCall();

void _init_mem()
{
	for(byte *src = &_DATA_START_FLASH, *dst = &_DATA_START; (byte*)dst < &_DATA_END; dst++, src++)
		*dst = *src;
	
	for(byte *dst = &_BSS_START; (byte*)dst < &_BSS_END; dst++)
		*dst = 0;
}

#ifdef CORTEX_M4
__attribute__((section(".text.init_vector")))
const void *const _vector[] = {
	&_STACK, _reset, _NMI, _hardfault,
	0, 0, 0, 0, 0, 0, 0, _SVCall
};
#else
#error No CPU chosen
#endif
