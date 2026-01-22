#include <stdlib.h>
#include "vec.h"

#define INITIAL_CAPACITY 16
#define ELEMENT_SIZE_SHIFT 3

struct vec_s {
    void **buffer;
    size_t len;
    size_t capacity;
};

vec vec_new() {
    vec v = malloc(sizeof(struct vec_s));
    v->buffer = malloc(INITIAL_CAPACITY << ELEMENT_SIZE_SHIFT);
    v->capacity = INITIAL_CAPACITY;
    v->len = 0;
    return v;
}

size_t vec_len(vec v) {
    return v->len;
}

void vec_double_capacity(vec v) {
    v->capacity <<= 1;
    v->buffer = realloc(v->buffer, v->capacity << ELEMENT_SIZE_SHIFT);
}

void *vec_get(vec v, size_t i) {
    return v->buffer[i];
}

void vec_set(vec v, size_t i, void *element) {
    v->buffer[i] = element;
}

void vec_push(vec v, void *element) {
    if (v->len == v->capacity) {
        vec_double_capacity(v);
    }

    vec_set(v, v->len, element);
    v->len++;
}

void *vec_pop(vec v) {
    return &v->buffer[--v->len];
}

void vec_free(vec v) {
    free(v->buffer);
    free(v);
}
