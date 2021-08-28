#include <stdio.h>
#include <string.h>

typedef struct _filme {
    char *nome;
    char *genero;
    int ano;
    float avaliacao;
} filme;

void busca_genero(filme *in, int tam, char *genero) {
    for (int i = 0; i < tam; i++) {
        if (strcmp(in[i].genero, genero) == 0) {
            printf("%s\n", in[i].nome);
        }
    }
}

int main() {
    filme vetor[2];
    vetor[0].nome = "Shrek";
    vetor[0].genero = "Aventura";
    vetor[0].ano = 2001;
    vetor[0].avaliacao = 5;

    vetor[1].nome = "Matrix";
    vetor[1].genero = "Ação";
    vetor[1].ano = 1999;
    vetor[1].avaliacao = 5;

    busca_genero(vetor, 2, "Aventura");

    return 0;
}

