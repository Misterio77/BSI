#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int maior_elemento_array(int *array, int tamanho) {
    int maior = INT_MIN;
    int i;
    for (i = 0; i < tamanho; i++) {
        if (array[i] > maior) {
            maior = array[i];
        }
    }
    return (maior);
}
long long int *gerar_fibonacci(int tamanho) {
    long long int *fib = (long long int *)malloc(sizeof(long long int)*tamanho);
    int i;
    for (i = 0; i < tamanho; i++) {
        if (i == 0) fib[i] = 0;
        else if (i == 1) fib[i] = 1;
        else fib[i] = fib[i-1] + fib[i-2];
    }
    return (fib);
}
int main(void) {
    int T;
    scanf("%d", &T);
    int *N = (int *)malloc(sizeof(int)*T);
    
    int i;
    for (i = 0; i < T; i++) {
        scanf("%d", &N[i]);
    }
    
    int maior = maior_elemento_array(N, T);
    long long int *fib = gerar_fibonacci(maior+1);
    
    for (i = 0; i < T; i++) {
        printf("Fib(%d) = %lld\n", N[i], fib[N[i]]);
    }

    free(fib);
    free(N);
}
