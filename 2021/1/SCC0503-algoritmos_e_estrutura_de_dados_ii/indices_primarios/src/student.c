#include "student.h"
#include "utils.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Dado arquivo, aloca e lê um estudante, o retornando
Student *student_from_file(FILE *input) {
    if (input == NULL) return NULL;

    Student *r = malloc(sizeof(Student));
    if (r == NULL) return NULL;

    // Lê cada um dos campos do estudante
    if (fread(&r->number, sizeof(int), 1, input) != 1
     || fread(&r->first_name, FIELD_LEN*sizeof(char), 1, input) != 1
     || fread(&r->last_name, FIELD_LEN*sizeof(char), 1, input) != 1
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
int student_to_file(Student *in, FILE *out) {
    if (in == NULL || out == NULL) return 0;
    if (fwrite(&in->number, sizeof(int), 1, out) != 1
     || fwrite(&in->first_name, FIELD_LEN*sizeof(char), 1, out) != 1
     || fwrite(&in->last_name, FIELD_LEN*sizeof(char), 1, out) != 1
     || fwrite(&in->major, FIELD_LEN*sizeof(char), 1, out) != 1
     || fwrite(&in->score, sizeof(float), 1, out) != 1) {
        // Caso algum falhe ao escrever, cancelar o resto
        return 0;
    }

    return 1;
}

// Dado uma linha de csv, aloca e deserializa um estudante, o retornando
Student *student_from_csv(const char *string) {
    if (string == NULL) return NULL;

    char *input = my_strdup((char *)string);

    // Usando calloc aqui ao invés de malloc pra garantir
    // que as strings estarão padded.
    //
    // Pq fazer isso? Pode ser que tenha lixo nelas depois do terminador.
    // Vai que tem algo sigiloso nessa memória? Melhor garantir.
    Student *r = calloc(1, sizeof(Student));
    if (r == NULL) return NULL;

    // Cortar com o delimitador usando strtok
    char *number_token = strtok(input, ",");
    if (number_token) {
        // Um ponteiro temporário
        // Normalmente ele serveria pra ver o que depois do número lido
        // Não ligamos pra isso, nesse caso.
        char *endptr;
        r->number = strtol(number_token, &endptr, 10);
        // Mas usaremos pra verificar se é igual ao number_token
        // Se sim, significa que não lemos nada e retornaremos erro.
        if (number_token == endptr) {
            free(input);
            student_destroy(r);
            return NULL;
        }
    } else {
        // Caso leitura de número falhe
        free(input);
        student_destroy(r);
        return NULL;
    }

    // Cortar primeiro nome com strtok
    char *first_name_token = strtok(NULL, ",");
    if (first_name_token) {
        my_strlcpy(r->first_name, first_name_token, FIELD_LEN);
    } else {
        // Caso leitura de nome falhe
        free(input);
        student_destroy(r);
        return NULL;
    }

    // Cortar sobrenome com strtok
    char *last_name_token = strtok(NULL, ",");
    if (first_name_token) {
        my_strlcpy(r->last_name, last_name_token, FIELD_LEN);
    } else {
        // Caso leitura de sobrenome falhe
        free(input);
        student_destroy(r);
        return NULL;
    }

    // Cortar curso com strtok
    char *major_token = strtok(NULL, ",");
    if (first_name_token) {
        my_strlcpy(r->major, major_token, FIELD_LEN);
    } else {
        // Caso leitura de curso falhe
        free(input);
        student_destroy(r);
        return NULL;
    }

    // Cortar com o delimitador usando strtok
    char *score_token = strtok(NULL, ",");
    if (score_token) {
        // Um ponteiro temporário
        // Normalmente ele serveria pra ver o que depois do número lido
        // Não ligamos pra isso, nesse caso.
        char *endptr;
        r->score = strtol(score_token, &endptr, 10);
        // Mas usaremos pra verificar se é igual ao score_token
        // Se sim, significa que não lemos nada e retornaremos erro.
        if (score_token == endptr) {
            free(input);
            student_destroy(r);
            return NULL;
        }
    } else {
        // Caso leitura de número falhe
        free(input);
        student_destroy(r);
        return NULL;
    }

    free(input);
    return r;
}

// Dado estudante, o desaloca completamente
void student_destroy(Student *input) {
    if (input != NULL) {
        free(input);
    }
}

// Dado estudante, exibe informações sobre ele na tela
void student_display(Student *input) {
    if (input == NULL) return;
    printf("-------------------------------\n");
    printf("USP number: %d\n", input->number);
    printf("Name: %s\n", input->first_name);
    printf("Surname: %s\n", input->last_name);
    printf("Course: %s\n", input->major);
    printf("Test grade: %.2f\n", input->score);
    printf("-------------------------------\n");
}

// Obter tamanho da estrutura, sem alinhamento
unsigned long student_size() {
    return(sizeof(int)
      +(50*sizeof(char))
      +(50*sizeof(char))
      +(50*sizeof(char))
          +sizeof(float));
}
