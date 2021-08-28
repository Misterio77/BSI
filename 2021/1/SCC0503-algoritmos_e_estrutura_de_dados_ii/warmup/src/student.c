#include "student.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Representa um estudante
struct _student {
    int number;
    char name[50];
    char major[50];
    float score;
};

// Dado arquivo, aloca e lê um estudante, o retornando
student *student_from_file(FILE *input) {
    student *r = (student *)malloc(sizeof(student));
    if (r == NULL) return NULL;

    // Lê cada um dos campos do estudante
    if (fread(&r->number, sizeof(int), 1, input) != 1
     || fread(&r->name, 50*sizeof(char), 1, input) != 1
     || fread(&r->major, 50*sizeof(char), 1, input) != 1
     || fread(&r->score, sizeof(float), 1, input) != 1) {
        // Caso alguma das leituras falhe, desalocar e retornar NULL
        student_destroy(r);
        return NULL;
    }
    return r;
}

// Dado estudante, o desaloca completamente
void student_destroy(student *input) {
    if (input != NULL) {
        free(input);
    }
}

// Dado estudante, exibe informações sobre ele na tela
void student_display(student *input) {
    printf("nUSP: %d\n", input->number);
    printf("Nome: %s\n", input->name);
    printf("Curso: %s\n", input->major);
    printf("Nota: %.2f\n", input->score);
}

// Obter tamanho da estrutura, sem alinhamento
unsigned long student_size() {
    return(sizeof(int)+(100*sizeof(char))+sizeof(float));
}
