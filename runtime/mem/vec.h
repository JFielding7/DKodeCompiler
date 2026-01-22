#ifndef VEC_H
#define VEC_H

typedef struct vec_s* vec;

vec vec_new();

size_t vec_len(vec v);

void *vec_get(vec v, size_t i);

void vec_set(vec v, size_t i, void *element);

void vec_push(vec v, void *element);

void *vec_pop(vec v);

void vec_free(vec v);

#endif //VEC_H
