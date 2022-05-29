#pragma once
#include "type.h"

struct AreaInfo
{
    word size: 31;
    bool taken: 1;
};

void _init_heap(word *start, word *end);
void *_alloc(word size, word align);
void _free(void *mem);
