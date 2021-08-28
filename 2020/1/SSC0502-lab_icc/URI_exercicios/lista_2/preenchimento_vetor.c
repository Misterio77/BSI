#include <stdio.h>
#include <stdlib.h>

int *preencher_vetor_dobro(int tamanho, int primeiro) {
    int *vetor = (int *)malloc(sizeof(int)*tamanho);
    int i;
    vetor[0] = primeiro;
    for (i = 1; i < tamanho; i++) {
        vetor[i] = vetor[i-1]*2;
    }
    return (vetor);
}

int main(void) {
    int V, i, *N;

    scanf("%d", &V);
    N = preencher_vetor_dobro(10, V);
    for (i = 0; i < 10; i++)
        printf("N[%d] = %d\n", i, N[i]);
    free(N);
}
    
