#include<stdio.h>
#include<stdlib.h>
#include<string.h>

#include"student.h"

#define BUFFER_SIZE 50

// Função para obter o tamanho de um arquivo
long fsize(FILE *file) {
    // Guardar posição atual (para retornarmos depois)
    long prev_pos = ftell(file);

    // Pular p/ final
    fseek(file, 0, SEEK_END);

    // Obter tamanho
    long size = ftell(file);

    // Voltar
    fseek(file, prev_pos, SEEK_SET);

    return size;
}

// Como todas as operações iniciam em algum ponto, imprimem, desalocam,
// e finalizam em outro ponto, essa função é uma versão genérica disso.
//
// Dado um arquivo, pula um offset 'offset', e lê até atingir 'limit'
void display_students_from_file(FILE *file, long offset, long limit) {
    // Partindo da posição atual, pular o offset especificado
    fseek(file, offset, SEEK_SET);

    // Marcar primeiro a ser printado (para não termos quebra de linha antes)
    int first = 1;

    student *current = NULL;
    // Iterar pelo arquivo. Caso o limite não tenha sido ultrapassado, ler e alocar o estudante
    while (ftell(file) < limit && (current = student_from_file(file)) != NULL) {
        // Imprimir quebra de linha antes, caso não seja o primeiro
        if (!first) {
            printf("\n");
        } else {
            first = 0;
        }

        // Imprimir e deslaocar
        student_display(current);
        student_destroy(current);
    }
}

// Lê uma (long) int do stdin
long int read_int() {
    char buffer[BUFFER_SIZE];

    fgets(buffer, BUFFER_SIZE, stdin);

    return strtol(buffer, NULL, 10);
}

// Lê uma string do stdin, com no máximo BUFFER_SIZE. Retorna um ponteiro pra ela alocada na heap.
char *read_string() {
    char *buffer = malloc(BUFFER_SIZE);
    if (buffer == NULL) return NULL;

    fgets(buffer, BUFFER_SIZE, stdin);

    // Retirar \n do final
    buffer[strcspn(buffer, "\n")] = '\0';

    return buffer;
}

int main() {
    char *path = read_string();
    FILE *file = fopen(path, "r");
    free(path);

    if (file == NULL) {
        printf("Não foi possível abrir o arquivo.");
        return(1);
    }

    int operation = read_int();

    long file_size = fsize(file);

    // Operação 1: todos os registros
    if (operation == 1) {
        long start_pos = 0;
        long end_pos = file_size;

        // Começar do início, ir até o fim
        display_students_from_file(file, start_pos, end_pos);
    }
    // Operação 2: registros do início até metade
    else if (operation == 2) {
        long start_pos = 0;
        long end_pos = file_size/2;

        // Começar do 0, ir até o meio
        display_students_from_file(file, start_pos, end_pos);
    }
    // Operação 3: registros da metade até o fim
    else if (operation == 3) {
        long start_pos = file_size/2;
        long end_pos = file_size;

        // Começar do meio, ir até o fim
        display_students_from_file(file, start_pos, end_pos);
    }
    // Operação 4: faixa
    else if (operation == 4) {
        int first = read_int();
        int last = read_int();

        // Começar do começo do primeiro, e acabar no final do último
        long start_pos = (first-1)*student_size();
        long end_pos = last*student_size();

        display_students_from_file(file, start_pos, end_pos);
    // Operação 5: apenas um
    } else if (operation == 5) {
        int chosen = read_int();

        // Começar do começo do pedido, e acabar no final dele mesmo
        long start_pos = (chosen-1)*student_size();
        long end_pos = chosen*student_size();

        display_students_from_file(file, start_pos, end_pos);
    } else {
        fclose(file);
        printf("Operação inválida.");
        return(2);
    }

    fclose(file);
}
