#ifndef DATABASE_H_
#define DATABASE_H_

#include "student.h"

#define DATABASE_FILE_PATH "database.dat"
#define INDEX_FILE_PATH "index.dat"
#define SLOTS_FILE_PATH "slots.dat"

typedef struct database Database;
void database_close(Database *database);
Database *database_open();
Student *database_search(Database *database, int number);
int database_insert(Database *database, Student *student);
int database_delete(Database *database, int number);
void database_sequential(Database *database);

#endif
