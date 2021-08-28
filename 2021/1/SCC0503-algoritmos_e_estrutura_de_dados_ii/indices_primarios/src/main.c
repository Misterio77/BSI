#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "database.h"
#include "student.h"

int main() {
    Database *database = database_open();

    char buffer[100];
    while (fgets(buffer, 100, stdin) != NULL) {
        // Retirar \n do final
        buffer[strcspn(buffer, "\n")] = '\0';

        // Pegar operação usando strtok
        // Ele irá transformar o primeiro espaço em \0
        char *operation = strtok(buffer, " ");
        if (operation == NULL) {
            // Caso não tenha nada na entrada, só ignorar e continuar o loop
            continue;
        } else if (strcmp(operation, "exit") == 0) {
            // Caso seja pedido pra sair, só quebrar o loop
            break;
        } else if (strcmp(operation, "insert") == 0) {
            // Somar buffer com strlen(operation) e +1 parece um pouquinho arcano,
            // mas não é tão complicado assim.
            //
            // Basicamente o strtok troca os espaços por caracteres nulos, e as operações relacionadas
            // a string sempre so consideram até o o nulo (strlen, strcmp).
            //
            // Daria pra pular pro próximo token usando strtok novamente
            // Mas isso iria quebrar entradas do csv que contém o separador (espaço)
            //
            // Então a ideia é usar aritmetica de ponteiro pra pular pro resto da string.
            // Tiro o tamanho da operacao, somo com 1, e pronto, agora temos um ponteiro
            // que contém tudo que vem depois da operação.
            char *csv = buffer+strlen(operation)+1;

            // Transformar csv em aluno
            Student *student = student_from_csv(csv);

            if (student == NULL) {
                printf("Input inválido!\n");
            } else {
                // Inserir
                if (database_insert(database, student) == 0) {
                    // Caso falhe com código 0, significa que já existe
                    printf("O Registro ja existe!\n");
                }
                student_destroy(student);
            }
        } else if (strcmp(operation, "search") == 0) {
            // Aritmetica de ponteiro para pular pra parte da string depois da operação
            char *number_string = buffer+strlen(operation)+1;
            // Converter string em int
            int number = strtol(number_string, NULL, 10);
            // Buscar na base de dados
            Student *student = database_search(database, number);
            // Caso nulo, significa que não foi encontrado
            if (student == NULL) {
                printf("Registro nao encontrado!\n");
            } else {
                student_display(student);
                student_destroy(student);
            }
        } else if (strcmp(operation, "delete") == 0) {
            // Aritmetica de ponteiro para pular pra parte da string depois da operação
            char *number_string = buffer+strlen(operation)+1;
            // Converter string em int
            int number = strtol(number_string, NULL, 10);
            // Remover da base de dados
            if (database_delete(database, number) == 0) {
                // Falhar com código 0 significa que não existe
                printf("Registro nao encontrado!\n");
            }
        } else if (strcmp(operation, "sequential") == 0) {
            // Imprimir sequencialmente
            database_sequential(database);
        } else {
            // Mesma coisa que a operação em branco, porém mostrando uma mensagem informativa
            printf("Operação inválida!\n");
        }
    }

    database_close(database);
}
