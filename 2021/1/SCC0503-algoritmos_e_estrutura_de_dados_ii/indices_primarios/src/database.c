#include "database.h"

#include "stack.h"
#include "utils.h"
#include "student.h"

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

// ===================
// Funções auxiliares
// ===================

// Item do índice (uso interno)
typedef struct index_item {
    // Chave primária
    int key;
    // RRN (posição no registro)
    long rrn;
} Index_item;

// Faz uma busca binária no índice, retornando a posição
long index_binary_search(Index_item *index, long left, long right, long key) {
    while (right >= left) {
        long middle = left + (right-left)/2;
        // Ver se encontramos no meio
        if (index[middle].key == key) {
            return middle;
        // Caso seja maior, descarta tudo à direita
        } else if (index[middle].key > key) {
            return index_binary_search(index, left, middle -1, key);
        // Caso seja menor, descarta tudo à esquerda
        } else {
            return index_binary_search(index, middle + 1, right, key);
        }
    }
    // Não encontrado
    return -1;
}

// Ordena o vetor de índices (com insertion sort, razoável pra vetores quase-ordenados)
void index_sort(Index_item *index, long index_len) {
    for (long i = 1; i < index_len; i++) {
        Index_item tmp = index[i];
        long j = i - 1;

        while (j >= 0 && tmp.key < index[j].key) {
            index[j+1] = index[j];
            j--;
        }
        index[j+1] = tmp;
    }
}


// ================
// Funções públicas
// ================

// State da database
struct database {
    // Arquivo da database
    FILE *file;

    // Índice
    Index_item *index;
    long index_len; // comprimento
    long index_cpc; // capacidade 

    // Pilha de espaços livres
    Stack *slots;
};

// Reconstruir índice (na memória apenas, deve fechar gracefully para salvar). O(n)
void index_rebuild(Database *database) {
    database->index_len = fsize(database->file)/student_size();
    database->index_cpc = max(database->index_len, 20);

    // Alocar índice, com o tamanho nescessário
    database->index = malloc(sizeof(struct index_item)*database->index_cpc);

    for (long i = 0; i < database->index_len; i++) {
        fseek(database->file, i*student_size(), SEEK_SET);
        Student *student = student_from_file(database->file);

        Index_item index_item;
        index_item.key = student->number;
        index_item.rrn = i;

        database->index[i] = index_item;
        student_destroy(student);
    }
    index_sort(database->index, database->index_len);
}


// Fecha a database, escrevendo o índice e a pilha nos arquivos
void database_close(Database *database) {
    if (database == NULL) return;

    // Fecha arquivo, caso aberto
    if (database->file != NULL) fclose(database->file);

    // Abre arquivo de índice para escrita (apaga antigo)
    FILE *index_file = fopen(INDEX_FILE_PATH, "w");
    if (index_file != NULL) {
        // Escrever byte de verificação (0 = ok, saída limpa e indice atualizado no arquivo)
        char byte = 0;
        fwrite(&byte, sizeof(char), 1, index_file);
        // Escrever cada par
        for (long i = 0; i < database->index_len; i++) {
            fwrite(&database->index[i].key, sizeof(int), 1, index_file);
            fwrite(&database->index[i].rrn, sizeof(long), 1, index_file);
        }
        free(database->index);
        fclose(index_file);
    }

    // Abre arquivo de slots para escrita (apaga antigo)
    FILE *slots_file = fopen(SLOTS_FILE_PATH, "w");
    if (slots_file != NULL) {
        while (!stack_is_empty(database->slots)) {
            // Esvaziar pilha no arquivo
            long data = stack_pop(&database->slots);
            fwrite(&data, sizeof(long), 1, slots_file);
        }
        fclose(slots_file);
    }

    free(database);
}

// Inicializa a database
Database *database_open() {
    Database *database = malloc(sizeof(Database));
    // Caso não seja possível alocar
    if (database == NULL) return NULL;

    // Inicializar
    database->file = NULL;
    database->slots = NULL;
    database->index = NULL;
    database->index_len = 0;
    database->index_cpc = 0;

    // == DATABASE ==
    // A função try_fopen (que eu defini em utils.c) tenta abrir em r+, e depois em w+ (caso falhe).
    // Esse arquivo permanecerá aberto enquanto a database estiver ativa
    database->file = try_fopen(DATABASE_FILE_PATH);
    // Caso não dê, sair com erro
    if (database->file == NULL) {
        database_close(database);
        return NULL;
    }
    
    // == ÍNDICE ==
    FILE *index_file = try_fopen(INDEX_FILE_PATH);
    // Caso não dê, sair com erro
    if (index_file == NULL) {
        database_close(database);
        return NULL;
    }

    // Ler o primeiro caracte, que vai ser o byte que define se é nescessário reconstrução
    char needs_rebuilding;
    // Caso esteja vazio ou precisemos reconstruir
    if (fread(&needs_rebuilding, sizeof(char), 1, index_file) != 1 || needs_rebuilding) {
        index_rebuild(database);
    } else {
        // Calcular tamanho atual do índice
        database->index_len = fsize(index_file)/(sizeof(int)+sizeof(long));

        // Quantidade de coisas já no arquivo de índice, ou 20, oq for maior
        // Tô implementando isso mais ou menos como um vector, pra evitar realocação
        database->index_cpc = max(database->index_len, 20);

        // Alocar índice, com o tamanho nescessário
        database->index = malloc(sizeof(struct index_item)*database->index_cpc);

        // Ler cada item do índice
        for (long i = 0; i < database->index_len; i++) {
            // Chave
            fread(&database->index[i].key, sizeof(int), 1, index_file);
            // RRN
            fread(&database->index[i].rrn, sizeof(long), 1, index_file);            
        }
    }

    // Fechar índice (o abriremos novamente, quando for nescessário escrever)
    fclose(index_file);

    // == SLOTS ==
    FILE *slots_file = try_fopen(SLOTS_FILE_PATH);
    // Caso não dê, sair com erro
    if (slots_file == NULL) {
        database_close(database);
        return NULL;
    }
    // Inicializar pilha
    database->slots = NULL;
    // Ler cada elemento da pilha de slots
    long rrn;
    while (fread(&rrn, sizeof(long), 1, slots_file) == 1) {
        // E ir adicionando à pilha
        stack_push(&database->slots, rrn);
    }
    // Fechar arquivo de slots (o abriremos novamente, caso seja nescessário escrever)
    fclose(slots_file);

    return database;
}

// Dado o número de um aluno, o busca na database e o retorna. (O(log n))
Student *database_search(Database *database, int number) {
    if (database == NULL) return NULL;

    if (database->index_len == 0) return NULL;
    long pos = index_binary_search(database->index, 0, database->index_len - 1, number);
    if (pos == -1) return NULL;

    long rrn = database->index[pos].rrn;

    fseek(database->file, rrn*student_size(), SEEK_SET);
    return student_from_file(database->file);
}

// Dado um estudante, o insere na database. O(n)
// 1 caso sucesso, 0 caso o registro já exista, -1 em outros casos
int database_insert(Database *database, Student *student) {
    if (student == NULL || database == NULL) {
        return -1;
    }
    Student *found = database_search(database, student->number);
    if (found != NULL) {
        student_destroy(found);
        return 0;
    }

    // Escrever byte de verificação (1 = indice ainda não foi salvo no arquivo)
    FILE *index_file = fopen(INDEX_FILE_PATH, "r+");
    if (index_file == NULL) return -2;
    char byte = 1;
    fwrite(&byte, sizeof(char), 1, index_file);
    fclose(index_file);

    long starting_point;
    // Caso a stack esteja vazia, pegar a ultima posição do arquivo
    if (stack_is_empty(database->slots)) {
        starting_point = fsize(database->file);
    // Caso tenhámos algo, usar aquela posição
    } else {
        long rrn = stack_pop(&database->slots);
        starting_point = rrn * student_size();
    }

    // Ir para a posição
    fseek(database->file, starting_point, SEEK_SET);
    // Escrever aluno na database
    if (student_to_file(student, database->file) == 0) return -2;
    fflush(database->file);

    // Aumentar e realocar índice, se nescessário
    database->index_len++;
    if (database->index_cpc < database->index_len) {
        database->index_cpc *= 1.5;
        // Caso o tamanho ultrapasse a capacidade, vou multiplicar a capacidade atual por 1.5
        // Isso garante um crescimento que faz sentido (20 -> 30 -> 45 -> 67 -> 100 -> 150...)
        // (Sim, eu tô meio que implementando um vector aqui, só pra não ter que realocar toda vez haha)
        database->index = realloc(database->index, sizeof(struct index_item)*(database->index_cpc));
    }

    // Criar item no índice
    Index_item index_item;
    index_item.key = student->number;
    index_item.rrn = starting_point/student_size();

    database->index[database->index_len-1] = index_item;

    // Vale a pena ordenar toda vez, já que pensamos num sistema otimizado para leitura
    // Isso possibilita busca binária
    index_sort(database->index, database->index_len);

    return 1;
}

// Dado o número de um aluno, o remove da database (caso exista). O(n)
int database_delete(Database *database, int number) {
    if (database == NULL) return -1;

    if (database->index_len == 0) return 0;
    long pos = index_binary_search(database->index, 0, database->index_len - 1, number);
    if (pos == -1) return 0;

    // Buscar rrn do aluno
    long rrn = database->index[pos].rrn;
    // Trocar sua chave no indice para  INT_MAX, efetivamente
    // retirando ele do vetor quando ordenarmos
    database->index[pos].key = INT_MAX;

    // Escrever byte de verificação (1 = indice ainda não foi salvo no arquivo)
    FILE *index_file = fopen(INDEX_FILE_PATH, "r+");
    if (index_file == NULL) return -2;
    char byte = 1;
    fwrite(&byte, sizeof(char), 1, index_file);
    fclose(index_file);

    stack_push(&database->slots, rrn);

    // Ordenar vetor (o recem removido vai pra ponta)
    index_sort(database->index, database->index_len);
    // Diminuir comprimento (efetivamente removendo o ultimo)
    database->index_len--;

    return 1;
}

// Percorre a database, imprimindo todos os alunos. O(n)
void database_sequential(Database *database) {
    fseek(database->file, 0, SEEK_SET);
    long n = fsize(database->file)/student_size();
    for (long i = 0; i < n; i++) {
        Student *student = student_from_file(database->file);
        student_display(student);
        student_destroy(student);
    }
}
