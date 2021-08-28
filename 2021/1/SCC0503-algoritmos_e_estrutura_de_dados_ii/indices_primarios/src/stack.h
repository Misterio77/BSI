#ifndef STACK_H_
#define STACK_H_

typedef struct stack Stack;
// Retorna 1 se a stack estiver vazia, 0 caso contrário
int stack_is_empty(const Stack *s);

// Retira a cabeça da stack e a retorna, -1 caso estiver vazia
long stack_pop(Stack **s);

// Insere um valor na cabeça da stack
int stack_push(Stack **s, long data);

#endif
