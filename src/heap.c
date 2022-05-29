#include "heap.h"

static struct AreaInfo *heap = 0;

void _init_heap(word *start, word *end)
{
    word size = end - start - 2;
    struct AreaInfo *header = (struct AreaInfo*)start;
    header->taken = 0;
    header->size = size / 4;
    struct AreaInfo *tail = (struct AreaInfo*)end - 1;
    tail->size = 0;
    heap = header;
}

void *_alloc(word size, word align)
{
    if(!heap)
        return NULL;
    word alignMask = align - 1;
    word words = size / 4 + (size % 4 != 0);
    struct AreaInfo *area = heap;
    word nextAlign = (word)(area + 1) & ~alignMask;
    if((word)(area + 1) & alignMask)
        nextAlign += align;
    word padWords = (nextAlign - (word)area) / 4 - 1;
    word actualSize = padWords + words;
    while(area->size && (area->taken || area->size < actualSize))
    {
        area = area + area->size + 1;
        nextAlign = (word)(area + 1) & ~alignMask;
        if((word)(area + 1) & alignMask)
            nextAlign += align;
        padWords = (nextAlign - (word)area) / 4 - 1;
        actualSize = padWords + words;
    }
    if(!area->size)
        return NULL;
    for(word *dst = (word*)area + 1; dst < (word*)nextAlign; dst++)
        *dst = 0;
    word remSize = area->size - actualSize - 1;
    if(remSize == 1)
        area->size = words + 1;
    else
    {
        struct AreaInfo *next = area + actualSize + 1;
        next->size = remSize;
        next->taken = 0;
        area->size = actualSize;
    }
    return (void*)nextAlign;
}

void _free(void *mem)
{
    struct AreaInfo *area = (struct AreaInfo*)mem - 1;
    while(!area->taken)
        area--;
    area->taken = 0;
    struct AreaInfo *next = area + area->size + 1;
    if(!next->taken)
        area->size += next->size + 1;
}
