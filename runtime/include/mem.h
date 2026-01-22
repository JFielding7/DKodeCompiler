#ifndef MEM_MANAGER_H
#define MEM_MANAGER_H

typedef struct gc_s *gc;

gc gc_new();

void allocate(const gc garbage_collector, size_t size);

void push_root(const gc collector, void *root);

void pop_root(const gc collector);

void collect(const gc garbage_collector);

void gc_free(const gc garbage_collector);

#endif
