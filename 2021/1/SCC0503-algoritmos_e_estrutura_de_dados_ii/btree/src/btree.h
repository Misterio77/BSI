#ifndef BTREE_H_
#define BTREE_H_

#include <stdbool.h>
#include <stdio.h>

// Ative para printar mensagens debug
#define DEBUG true

// Tamanho da página de disco
#define PAGESIZE 4096
// Número máximo de registros em um nó
#define MAXKEYS 204
//#define MAXKEYS 3

// Retorna o valor correspondente a 'key', caso exista
// Se não, retorna -1
long btree_search(int key, FILE *file);

// Insere um registro
// Retorna false se não for possível
bool btree_insert(int key, long value, FILE *file);
#endif
