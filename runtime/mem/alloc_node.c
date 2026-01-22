#include <stddef.h>
#include <stdlib.h>

#include "alloc_node.h"

alloc_node alloc_node_new(const size_t size) {
    const alloc_node node = malloc(sizeof(struct alloc_node_s) + size);
    node->next = NULL;
    return node;
}

alloc_node get_node(void *mem) {
    return mem - sizeof(struct alloc_node_s);
}

void *get_mem(const alloc_node node) {
    return node + 1;
}
