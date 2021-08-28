#include<stdio.h>
#include<stdlib.h>
#include<string.h>

#include"student.h"

int main() {
    // Buffer para ler as entradas
    char buffer[100];
    // Abrir arquivo para escrever
    FILE *out = fopen("dados.dat", "w");
    // Ler as linhas
    while (fgets(buffer, 100, stdin) != NULL) {
        // Parse
        student *tmp = student_from_csv(buffer);
        // Escrever
        if (tmp == NULL) {
            // Caso a leitura de um estudante completo falhe
            fprintf(stderr, "Erro ao ler estudante, pulando.\n");
        } else {
            student_to_file(tmp, out);
        }
        // Desalocar
        student_destroy(tmp);
    }
    fclose(out);

    // Abrir arquivo para ler
    FILE *in = fopen("dados.dat", "r");
    // Pular o décimo aluno (contando do fim) no arquivo
    fseek(in, student_size()*(-10), SEEK_END);

    student *tmp;
    // Marcar primeiro (para não pular linha a +)
    int first = 1;
    // Ler cada aluno
    while ((tmp = student_from_file(in)) != NULL) {
        if (first) {
            first = 0;
        } else {
            printf("\n");
        }
        // Exibir
        student_display(tmp);
        // Desalocar
        student_destroy(tmp);
    }
    fclose(in);
}
