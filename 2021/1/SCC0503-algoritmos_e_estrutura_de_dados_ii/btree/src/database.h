#ifndef DATABASE_H_
#define DATABASE_H_

#include <stdbool.h>
#include "student.h"

// Arquivo de dados
#define DATABASE_FILE_PATH "database.dat"
// Arquivo de índice (btree)
#define INDEX_FILE_PATH "index.dat"

// Estado da database 
typedef struct database Database;
// Fechar database
void database_close(Database *database);
// Abrir database
Database *database_open();
// Buscar estudante
Student *database_search(Database *database, int number);
// Inserir estudante
bool database_insert(Database *database, Student *student);
// Atualizar estudante
bool database_update(Database *database, Student *student);

#endif
