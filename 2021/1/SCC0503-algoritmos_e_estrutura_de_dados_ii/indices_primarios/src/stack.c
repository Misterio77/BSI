#include<stdlib.h>
#include"stack.h"

/* 
 * Implementação de pilha
 * Utilizada para pilha de remoção.
 */

struct stack {
    long data;
    Stack *next;
};

// Retorna 1 se a stack estiver vazia, 0 caso contrário
int stack_is_empty(const Stack *s) {
    return (s == NULL);
}

// Retira a cabeça da stack e a retorna, -1 caso estiver vazia
long stack_pop(Stack **s) {
    Stack *tmp;
    long data;

    if (stack_is_empty(*s)) {
        return(-1);
    }

    // Armazenar nó atual
    tmp = *s;

    // Trocar nó da stack pro próximo
    *s = (*s)->next;

    // Pegar dado do nó
    data = tmp->data;
    // Desalocar nó
    free(tmp);

    return(data);
}

// Insere um valor na cabeça da stack
// 1 se der tudo certo, 0 caso contrário
int stack_push(Stack **s, long data) {
    // Criar nó
    Stack *new = malloc(sizeof(Stack));
    if (new == NULL) return 0;

    // Guardar dado no novo nó
    new->data = data;
    // Marcar o próximo dele como a cabeça da atual
    new->next = *s;
    // Trocar cabeça para o novo
    *s = new;

    return 1;
}
