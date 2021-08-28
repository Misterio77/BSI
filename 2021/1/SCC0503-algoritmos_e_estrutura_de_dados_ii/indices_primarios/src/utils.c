#include<string.h>
#include<stdio.h>
#include<stdlib.h>

// Meio dificil ter um strlcpy consistente, então trouxe a implementação de um pra cá
// Implementação do OpenBSD (https://github.com/libressl-portable/openbsd/blob/master/src/lib/libc/string/strlcpy.c)
long my_strlcpy(char *dst, const char *src, long dsize) {
	const char *osrc = src;
	long nleft = dsize;

	/* Copy as many bytes as will fit. */
	if (nleft != 0) {
		while (--nleft != 0) {
			if ((*dst++ = *src++) == '\0')
				break;
		}
	}

	/* Not enough room in dst, add NUL and traverse rest of src. */
	if (nleft == 0) {
		if (dsize != 0)
			*dst = '\0';		/* NUL-terminate dst */
		while (*src++)
			;
	}

	return(src - osrc - 1);	/* count does not include NUL */
}

// Strdup é posix, então trouxe uma pra cá também
char *my_strdup(char *src) {
    char *str;
    char *p;
    int len = 0;

    while (src[len])
        len++;
    str = malloc(len + 1);
    p = str;
    while (*src)
        *p++ = *src++;
    *p = '\0';
    return str;
}

// Função para obter o tamanho de um arquivo (a partir do ponto atual)
unsigned long fsize(FILE *file) {
    // Guardar posição atual (para retornarmos depois)
    unsigned long prev_pos = ftell(file);

    // Pular p/ final
    fseek(file, 0, SEEK_END);

    // Obter tamanho
    unsigned long size = ftell(file);

    // Voltar
    fseek(file, prev_pos, SEEK_SET);

    return size;
}

// Tenta abrir um arquivo com r+. Caso falhe, tenta com w+.
FILE *try_fopen(const char *filename) {
    FILE *file = fopen(filename, "r+");
    if (file == NULL) {
        file = fopen(filename, "w+");
        if (file == NULL) return NULL;
    }

    return file;
}

// Máximo entre dois números
long max(long a, long b) {
    return (a > b) ? a : b;
}
