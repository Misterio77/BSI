#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>
#include <limits.h>
#include "btree.h"

// Tamanho do header do arquivo
#define HEADERSIZE PAGESIZE

// Tamanho dos campos auxiliares no nó
#define AUX_FIELDS_SIZE_ON_PAGE (sizeof(int)+sizeof(char))
// Tamanho de cada elemento
#define RECORDSIZE (sizeof(int)+sizeof(long))
// Tamanho do índice usado para os nós
#define INDEXSIZE (sizeof(long))

// Tamanho do nó
#define NODESIZE ((MAXKEYS*RECORDSIZE) + ((MAXKEYS+1)*INDEXSIZE) + AUX_FIELDS_SIZE_ON_PAGE)
// Tamanho que deve sobrar depois do nó, para alinhar em páginas
#define FREE_SPACE_ON_PAGE (PAGESIZE - NODESIZE)

// Dado armazenado na btree (um índice)
typedef struct record {
    // Chave primária (nusp)
    int key;
    // Valor (RRN no arquivo de dados)
    long value;
} Record;

// Uma página (nó) da btree
typedef struct page {
    // Se um nó é folha
    bool is_leaf;
    // Número de registros no nó
    int number;
    // rrn dos nós filhos
    long *children;
    // Registros (indices) naquele nó
    Record *records;
} Page;

// =================
// Funções da page
// =================
// Desalocar página
void page_destroy(Page *page) {
    if (!page) return;
    if (page->children) free(page->children);
    if (page->records) free(page->records);
    free(page);
}

// Lê página de um arquivo (assume que o cursor está no inicio do registro)
Page *page_read(long page_rrn, FILE *file) {
    fseek(file, HEADERSIZE+(page_rrn*PAGESIZE), SEEK_SET);
    // Verificar se arquivo não é NULL
    if (file == NULL) return NULL;
    // Alocar página, padeada com zero (para não ter undefined behaviour na hora de destruir)
    Page *r = malloc(sizeof(Page));
    r->is_leaf = false;
    r->number = 0;
    r->children = NULL;
    r->records = NULL;

    if (r == NULL) return NULL;

    char is_leaf;
    // Ler se é folha
    if (fread(&is_leaf, sizeof(char), 1, file) != 1) {
        page_destroy(r);
        return NULL;
    }
    r->is_leaf = (is_leaf);

    int number;
    // Ler número de registros
    if (fread(&number, sizeof(int), 1, file) != 1) {
        // Caso não consiga ler
        page_destroy(r);
        return NULL;
    }
    r->number = number;

    // Alocar vetor de filhos
    r->children = malloc((r->number+1)*sizeof(long));
    if (r->children == NULL) {
        page_destroy(r);
        return NULL;
    }

    // Ler índices dos nós filhos
    for (int i = 0; i < r->number+1; i++) {
        if (fread(&r->children[i], sizeof(long), 1, file) != 1) {
            page_destroy(r);
            return NULL;
        }
    }
    // Pular o espaço até o fim dos filhos
    fseek(file, (MAXKEYS-(r->number))*INDEXSIZE, SEEK_CUR);


    // Alocar registros
    r->records = malloc(r->number*sizeof(Record));
    if (r->records == NULL) {
        page_destroy(r);
        return NULL;
    }

    // Ler cada registro
    for (int i = 0; i < r->number; i++) {
        int key;
        long value;
        // Ler cada campo do registro
        if (fread(&key, sizeof(int), 1, file) != 1
         || fread(&value, sizeof(long), 1, file) != 1) {
            // Caso alguma leitura falhe
            page_destroy(r);
            return NULL;
        }

        Record record;
        record.key = key;
        record.value = value;
        // Guardar no vetor de registros
        r->records[i] = record;
    }

    // Pular para fim do espaço alocado para os registros
    // Assim, se o máximo é 5 e temos 2, pular 3 posições pra frente
    fseek(file, (MAXKEYS-(r->number))*RECORDSIZE, SEEK_CUR);

    // Pular parte vazia depois do nó
    fseek(file, FREE_SPACE_ON_PAGE, SEEK_CUR);

    return r;
}

// Escreve a página para um arquivo (assume que o cursor está no inicio de sua posição) 
// 1 se der tudo certo, 0 caso contrário
bool page_write(Page *page, long page_rrn, FILE *file) {
    fseek(file, HEADERSIZE+(page_rrn*PAGESIZE), SEEK_SET);
    if (file == NULL || page == NULL) return false;
    if (fwrite(&page->is_leaf, sizeof(char), 1, file) != 1
     || fwrite(&page->number, sizeof(int), 1, file) != 1) {
        return false;
    }

    for (int i = 0; i < page->number+1; i++) {
        if (fwrite(&page->children[i], sizeof(long), 1, file) != 1) {
            return false;
        }
    }

    // Pular o espaço até o fim dos filhos
    fseek(file, (MAXKEYS-(page->number))*sizeof(long), SEEK_CUR);

    for (int i = 0; i < page->number; i++) {
        if (fwrite(&page->records[i].key, sizeof(int), 1, file) != 1
         || fwrite(&page->records[i].value, sizeof(long), 1, file) != 1) {
            return false;
        }
    }
    // Pular para fim do espaço alocado para os registros
    // Assim, se o máximo é 5 e temos 2, pular 3 posições pra frente
    fseek(file, (MAXKEYS-(page->number))*RECORDSIZE, SEEK_CUR);

    // Pular parte vazia depois do nó
    fseek(file, FREE_SPACE_ON_PAGE, SEEK_CUR);
    return true;
}

// Insere registro numa página não-cheia, movendo os registros e filhas conforme nescessário 
void page_insert_record(Page *page, int key, long value, long child) {
    // Aqui temos certeza que irá caber, pq fomos partindo as filhas conforme descemos

    // Atualizar numero de posições
    page->number += 1;

    // Realocar vetores de registros e filhos
    page->records = realloc(page->records, (page->number)*sizeof(Record));
    page->children = realloc(page->children, (page->number+1)*sizeof(long));

    // Procurar um lugar pra ele
    int pos;
    for (pos = 0; pos < page->number; pos++) {
        // Caso encontre uma posição que caiba ou seja a ultima (vazia)
        if (pos == page->number-1 || page->records[pos].key > key) {
            break;
        }
    }

    // Abrir espaço pro novo registro, movendo todos
    // que vêm depois dele uma casa pra direita
    for (int i = page->number-1; i > pos; i--) {
        page->records[i] = page->records[i-1];
    }
    // Mover conexões pra direita tb
    for (int i = page->number; i > pos+1; i--) {
        page->children[i] = page->children[i-1];
    }

    // Adicionar dados
    page->records[pos].key = key;
    page->records[pos].value = value;
    page->children[pos+1] = child;
}

// Corta uma pagina, dada seu pai (pode passar nulo, se não houver)
// O outro pedaço irá para o final do indice
// Retorna o rrn do nó pai
// Supõe que o cursor está antes de 'page'
long page_split(Page *page, long page_rrn, Page *parent, FILE *file) {
    // Criar nova pagina
    Page *new_page = malloc(sizeof(Page));
    if (!new_page) return -1;
    new_page->is_leaf = page->is_leaf;

    // Anotar tamanho antigo do nó
    int old_size = page->number;

    // Pegar mediana dele
    int median = (old_size-1)/2;

    // Colocar (n-1)/2 no tamanho da nova pagina e da atual
    new_page->number = median;
    page->number = median;

    new_page->children = malloc((new_page->number)*sizeof(long));
    new_page->records = malloc((new_page->number)*sizeof(Record));

    // Pegar registro que fica na mediana
    Record median_record = page->records[median];

    // Pular para o fim do arquivo
    fseek(file, 0, SEEK_END);

    // Pegar rrn onde adicionaremos
    long new_rrn = (ftell(file)-HEADERSIZE)/PAGESIZE;

    // Caso seja raiz
    if (!parent) {
        // Criar pai e apontar o lado esquerdo para 'page'
        parent = malloc(sizeof(Page));
        if (!parent) return -1;
        parent->is_leaf = false;
        parent->number = 0;
        parent->children = malloc((parent->number+1)*sizeof(long));
        parent->children[0] = page_rrn;
        parent->records = NULL;
        // Escrever nó raiz
        page_write(parent, new_rrn, file);

        // O rrn que retornaremos será o dessa raiz
        page_rrn = new_rrn;

        // Como escrevemos um nó agora,
        // o nó que criaremos logo mais vai
        // 1 rrn pra frente
        new_rrn++;
    }

    // Mover os registros a direita da mediana para a nova pagina
    for (int i = old_size-1; i > median; i--) {
        new_page->records[i-median-1] = page->records[i];
    }
    // Mover as conexões a direita da mediana para a nova pagina
    for (int i = old_size; i > median; i--) {
        new_page->children[i-median-1] = page->children[i];
    }

    // Botar nova página no final da btree
    page_write(new_page, new_rrn, file);

    // Adicionar registro que estava no meio ao pai
    // E também apontar o lado direito desse registro para o novo nó
    // (Essa função não escreve no arquivo, só na memória)
    page_insert_record(parent, median_record.key, median_record.value, new_rrn);

    // Deslocar da memória do nó splitado
    page_destroy(new_page);


    return page_rrn;
}


// =================
// Funções internas da btree
// =================

// Altera o nó raiz no cabeçalho
bool btree_set_root(long root_rrn, FILE *file) {
    fseek(file, 0, SEEK_SET);
    if (fwrite(&root_rrn, sizeof(long), 1, file) != 1) {
        return false;
    }

    fseek(file, HEADERSIZE, SEEK_SET);
    return true;
}

// Retorna o nó raiz do cabeçalho
long btree_get_root(FILE *file) {
    fseek(file, 0, SEEK_SET);
    long root_rrn;
    // Ler rrn do nó do cabeçalho do arquivo
    if (fread(&root_rrn, sizeof(long), 1, file) != 1) {
        // Caso não seja possível ler, vamos assumir (e escrever) zero
        root_rrn = 0;
        btree_set_root(root_rrn, file);
    }
    // Pular para o fim do cabeçalho
    fseek(file, HEADERSIZE, SEEK_SET);

    return root_rrn;
}

// Partindo de um nó 'page', retorna o valor que corresponde a uma chave
// -1 caso não encontre
long btree_search_inner(Page *page, int key, FILE *file) {
    // Caso seja um nó vazio (root inicial)
    if (page->number == 0) {
        page_destroy(page);
        return -1;
    }
    // Iterar pelos registros, buscando o mais próximo da nossa busca
    // Índice do mais próximo
    int closest = 0;
    for (int i = 0; i < page->number; i++) {
        // Caso o atual esteja mais próximo da chave que o 'closest' está
        if (abs(page->records[i].key - key) < abs(page->records[closest].key - key)) {
            // Se sim, colocar indice no closest
            closest = i;
        }
    }

    Record closest_record = page->records[closest];
    // Caso tenhamos encontrado
    if (closest_record.key == key) {
        long value = closest_record.value;
        page_destroy(page);
        return value;
    } else {
        // Caso chegamos na folha, não deu pra achar
        if (page->is_leaf) {
            page_destroy(page);
            return -1;
        }

        // Índice da próxima página no vetor de nós filhos
        int next_page_index;
        // Caso o mais próximo seja menor que a buscada
        if (closest_record.key < key) {
            // Ir para o filho com o mesmo indice (vulgo, à esquerda)
            next_page_index = closest;
        } else {
            // Ir para o filho com indice +1 (vulgo, à direita)
            next_page_index = closest+1;
        }

        // Pegar o RRN
        long rrn = page->children[next_page_index];
        // Desalocar página anterior
        page_destroy(page);

        // Pular para e ler nó filho desejado
        Page *next_page = page_read(rrn, file);

        // Chamar novamente, na pagina filha
        return btree_search_inner(next_page, key, file);
    }
}

// Dado nó, insere um 'value' numa 'key'
// Presupoe que o cursor está no começo do nó
bool btree_insert_inner(Page *current, long current_rrn, int key, long value, FILE *file) {
    // Caso seja folha, inserir
    // Aqui temos certeza que há espaço, pois cada recursão rachou
    // a filha da página passada
    if (current->is_leaf) {
        for (int i = 0; i <= current->number; i++) {
            current->children[i] = -1;
        }
        // Inserir registro
        page_insert_record(current, key, value, -1);
        // Escrever página e retornar
        bool ok = page_write(current, current_rrn, file);
        page_destroy(current);
        return ok;
    }

    // Buscar o próximo filho que iremos
    // Iterar pelos registros, buscando o mais próximo da nossa busca
    
    // Índice do mais próximo
    int closest = 0;
    for (int i = 0; i < current->number; i++) {
        // Caso o atual esteja mais próximo da chave que o 'closest' está
        if (abs(current->records[i].key - key) < abs(current->records[closest].key - key)) {
            // Se sim, colocar indice no closest
            closest = i;
        }
    }

    // Pegar o nó mais próximo da nossa chave
    Record closest_record = current->records[closest];

    // Caso seja oq queremos inserir
    // Não precisaremos splitar e podemos acabar por aqui
    if (closest_record.key == key) {
        // Atualizar valor existente
        closest_record.value = value;
        // Escrever no arquivo
        return page_write(current, current_rrn, file);
    }

    // Vamos selecionar pra qual filho ir
    // Índice da próxima página no vetor de nós filhos
    int child_index;
    // Caso o mais próximo seja menor que a buscada
    if (closest_record.key < key) {
        // Ir para o filho com o mesmo indice (vulgo, à esquerda)
        child_index = closest;
    } else {
        // Ir para o filho com indice +1 (vulgo, à direita)
        child_index = closest+1;
    }

    // Pegar o RRN do filho que selecionamos
    long child_rrn = current->children[child_index];

    // Ler a próxima página
    Page *next_page = page_read(child_rrn, file);

    // Caso ela esteja cheia
    if (next_page->number == MAXKEYS) {
        // Rachar esse novo nó (passando tb seu pai, vulgo current)
        page_split(next_page, child_rrn, current, file);

        // Escrever pagina do nó pai
        page_write(current, current_rrn, file);

        // Escrever página do nó filho
        page_write(next_page, child_rrn, file);

        // Desalocar proxima pagina, não a usaremos mais
        page_destroy(next_page);

        // Chamar novamente, nessa mesma página. Para o caso de ter mudado as filhas
        return btree_insert_inner(current, current_rrn, key, value, file);
    }

    // Desalocar página, não a usaremos mais
    page_destroy(current);

    // Chamar novamente na pagina filha
    return btree_insert_inner(next_page, child_rrn, key, value, file);

}

// =================
// Funções públicas da btree
// =================

// Retorna o valor correspondente a 'key', caso exista
// Se não, retorna -1
long btree_search(int key, FILE *file) {
    // Pegar rrn da raiz
    long root_rrn = btree_get_root(file);
    // Ler ela na memoria
    Page *root = page_read(root_rrn, file);
    if (!root) {
        return -1;
    }

    // Chamar função interna de busca, recursiva
    long value = btree_search_inner(root, key, file);
    return value;
}

// Insere um registro
// Retorna false se não for possível
bool btree_insert(int key, long value, FILE *file) {
    // Pegar rrn da raiz
    long root_rrn = btree_get_root(file);
    // Ler ela na memoria
    Page *root = page_read(root_rrn, file);
    if (!root) {
        root = malloc(sizeof(Page));
        if (!root) return -1;
        root->is_leaf = true;
        root->number = 0;
        root->children = malloc((root->number+1)*sizeof(long));
        root->children[0] = -1;
        root->records = NULL;
        // Escrever nó raiz
        page_write(root, root_rrn, file);
    }

    // Caso a raiz esteja cheia, vamos rachar ela
    // (é um caso especial, pois o inner só consegue rachar o filho da passada)
    if (root->number == MAXKEYS) {
        // Rachar página e pegar o rrn da nova raiz
        long new_root_rrn = page_split(root, root_rrn, NULL, file);
        // Anotar no arquivo
        btree_set_root(root_rrn, file);

        // Atualizar antiga raiz
        page_write(root, root_rrn, file);
        // Desalocar ela
        page_destroy(root);

        // Ler a nova raiz
        root = page_read(new_root_rrn, file);
    }

    // Chamar a função interna, recursiva
    bool ok = btree_insert_inner(root, root_rrn, key, value, file);
    return ok;
}
