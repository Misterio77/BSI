#ifndef UTILS_H_
#define UTILS_H_

#include <stdio.h>

long my_strlcpy(char *dst, const char *src, long dsize);
char *my_strdup(char *src);
long fsize(FILE *file);
FILE *try_fopen(const char *filename);
unsigned max(unsigned a, unsigned b);

#endif
