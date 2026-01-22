#include <stdlib.h>
#include "../include/mem.h"
#include "alloc_node.h"
#include "vec.h"

struct gc_s {
    vec ptr_stack;
    alloc_node head;
    alloc_node tail;
};

gc gc_new() {
    const gc garbage_collector = malloc(sizeof(struct gc_s));

    garbage_collector->head = alloc_node_new(0);
    garbage_collector->tail = garbage_collector->head;

    return garbage_collector;
}

void allocate(const gc garbage_collector, size_t size) {
    const alloc_node node = alloc_node_new(size);
    garbage_collector->tail->next = node;
    garbage_collector->tail = node;
}

void push_root(const gc collector, void *root) {
    vec_push(collector->ptr_stack, root);
}

void pop_root(const gc collector) {
    vec_pop(collector->ptr_stack);
}

void mark(const gc garbage_collector) {
    const size_t len = vec_len(garbage_collector->ptr_stack);

    for (size_t i = 0; i < len; i++) {
        const alloc_node node = vec_get(garbage_collector->ptr_stack, i);
        node->in_use = true;
    }
}

void sweep(const gc garbage_collector) {
    alloc_node prev = garbage_collector->head;
    alloc_node curr = prev->next;

    while (curr) {
        const alloc_node next = curr->next;

        if (curr->in_use) {
            curr->in_use = false;
        } else {
            free(curr);
            prev->next = next;
        }

        prev = curr;
        curr = next;
    }
}

void collect(const gc garbage_collector) {
    mark(garbage_collector);
    sweep(garbage_collector);
}

void gc_free(const gc garbage_collector) {
    vec_free(garbage_collector->ptr_stack);
    free(garbage_collector->head);
    free(garbage_collector);
}
