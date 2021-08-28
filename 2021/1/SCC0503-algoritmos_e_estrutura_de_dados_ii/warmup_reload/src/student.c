#include "student.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FIELD_LEN 50

// Representa um estudante
struct _student {
    int number;
    char name[FIELD_LEN];
    char major[FIELD_LEN];
    float score;
};

// Dado arquivo, aloca e lê um estudante, o retornando
student *student_from_file(FILE *input) {
    if (input == NULL) return NULL;

    student *r = (student *)malloc(sizeof(student));
    if (r == NULL) return NULL;

    // Lê cada um dos campos do estudante
    if (fread(&r->number, sizeof(int), 1, input) != 1
     || fread(&r->name, FIELD_LEN*sizeof(char), 1, input) != 1
     || fread(&r->major, FIELD_LEN*sizeof(char), 1, input) != 1
     || fread(&r->score, sizeof(float), 1, input) != 1) {
        // Caso alguma das leituras falhe, desalocar e retornar NULL
        student_destroy(r);
        return NULL;
    }
    return r;
}

// Escreve um estudante num arquivo
// Retorna 1 caso tenha sucesso, 0 caso contrário
int student_to_file(student *in, FILE *out) {
    if (in == NULL || out == NULL) return 0;
    if (fwrite(&in->number, sizeof(int), 1, out) != 1
     || fwrite(&in->name, FIELD_LEN*sizeof(char), 1, out) != 1
     || fwrite(&in->major, FIELD_LEN*sizeof(char), 1, out) != 1
     || fwrite(&in->score, sizeof(float), 1, out) != 1) {
        // Caso algum falhe ao escrever, cancelar o resto
        return 0;
    }

    return 1;
}

// Dado uma linha de csv, aloca e deserialize um estudante, o retornando
// A string de input é consumida
student *student_from_csv(char *input) {
    if (input == NULL) return NULL;

    student *r = (student *)malloc(sizeof(student));
    if (r == NULL) return NULL;

    // Começar do primeiro campo, número
    // Cortar com o delimitador usando strtok
    char *number_token = strtok(input, ",");
    // Tentar ler com sscanf
    if (number_token == NULL || sscanf(number_token, "%d", &r->number) != 1) {
        // Caso falhe
        student_destroy(r);
        return NULL;
    }

    // Cortar nome com strtok
    char *name_token = strtok(NULL, ",");
    // Copiar do token
    if (name_token == NULL || strncpy(r->name, name_token, FIELD_LEN) == NULL) {
        // Caso falhe
        student_destroy(r);
        return NULL;
    }

    // Cortar curso com strtok
    char *major_token = strtok(NULL, ",");
    // Copiar do token
    if (major_token == NULL || strncpy(r->major, major_token, FIELD_LEN) == NULL) {
        // Caso falhe
        student_destroy(r);
        return NULL;
    }

    // Cortar nota com strtok
    char *score_token = strtok(NULL, ",");
    // Tentar ler com sscanf
    if (score_token == NULL || sscanf(score_token, "%f", &r->score) != 1) {
        // Caso falhe
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
