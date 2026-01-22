#ifndef ALLOC_NODE_H
#define ALLOC_NODE_H

#include <stdbool.h>

typedef struct alloc_node_s* alloc_node;

struct alloc_node_s {
    bool in_use;
    alloc_node next;
    // allocated memory
};

alloc_node alloc_node_new(size_t size);

void *get_mem(alloc_node node);

#endif
