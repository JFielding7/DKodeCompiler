#ifndef STRING_H
#define STRING_H

#include <stddef.h>

struct str_s {
    char *buffer;
    size_t length;
};

typedef struct str_s *str;

str str_new(const char *chars, size_t length);

void str_print(str string);

#endif //STRING_H
