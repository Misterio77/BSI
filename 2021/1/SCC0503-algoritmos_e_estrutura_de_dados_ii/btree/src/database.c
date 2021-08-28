#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

#include "database.h"
#include "student.h"
#include "btree.h"
#include "utils.h"

struct database {
    FILE *index;
    FILE *data;
};

Database *database_open() {
    // Abrir arquivos

    FILE *index = try_fopen(INDEX_FILE_PATH);
    if (!index) {
        printf("Não foi possível abrir o arquivo de indice\n.");
        return NULL;
    }
    FILE *data = try_fopen(DATABASE_FILE_PATH);
    if (!data) {
        printf("Não foi possível abrir o arquivo de dados\n.");
        fclose(index);
        return NULL;
    }

    Database *database = malloc(sizeof(Database));
    if (!database) {
        printf("Não foi possível alocar database\n");
        fclose(index);
        fclose(data);
        return NULL;
    }
    database->index = index;
    database->data = data;

    return database;
}

void database_close(Database *database) {
    // Fechar arquivos
    if (!database) return;
    if (database->index) fclose(database->index);
    if (database->data) fclose(database->data);
    free(database);
}

Student *database_search(Database *database, int number) {
    // Buscar rrn na btree indice
    long rrn = btree_search(number, database->index);
    if (rrn == -1) {
        printf("Registro nao encontrado!\n");
        return NULL;
    }
    fseek(database->data, rrn*STUDENT_SIZE, SEEK_SET);
    // Pegar aluno e o retornar
    Student *student = student_from_file(database->data);
    if (student == NULL) {
        printf("Registro nao encontrado!\n");
        return NULL;
    }
    return student;
}

bool database_insert(Database *database, Student *student) {
    if (!student) {
        printf("Estudante inválido!\n");
        return false;
    }
    long search_rrn = btree_search(student->number, database->index);
    if (search_rrn != -1) {
        printf("O Registro ja existe!\n");
        return false;
    }
    // Pular pro final da database
    fseek(database->data, 0, SEEK_END);
    if(!student_to_file(student, database->data)) {
        printf("Erro ao escrever no arquivo %s\n", DATABASE_FILE_PATH);
        return false;
    }
    long rrn = (ftell(database->data)/STUDENT_SIZE)-1;
    return btree_insert(student->number, rrn, database->index);
}

bool database_update(Database *database, Student *student) {
    // Buscar rrn na btree indice
    long rrn = btree_search(student->number, database->index);
    if (rrn == -1) {
        printf("Registro nao encontrado!\n");
        return NULL;
    }
    // Pular para o registro
    fseek(database->data, rrn*STUDENT_SIZE, SEEK_SET);
    // Escrever
    return student_to_file(student, database->data);
}
