#include "../include/str.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

str str_new(const char *chars, const size_t length) {
    const str string = malloc(sizeof(struct str_s));

    char *buffer = malloc(length);
    memcpy(buffer, chars, length);

    string->buffer = buffer;
    string->length = length;

    return string;
}

void str_print(const str string) {
    puts("dk won");
    printf("%.*s", (int) string->length, string->buffer);
}
