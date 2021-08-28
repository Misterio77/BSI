#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

long long int fib(int n, int *cont, int b) {
    *cont += 1;
    printf(".");
    if (n == 0) return (0);
    else if (n == 1) return (1);
    else return (fib(n-1, cont, b)+fib(n-2, cont, b));
}

int contar_chamadas(int alvo, int base) {
    int chamadas = 0;
    fib(alvo, &chamadas, base);
    return (chamadas);
}

int main() {
    int N = 0;
    int B = 0;
    int casos = 0;
    do {
        scanf("%d %d", &N, &B);
        printf("Case %d: ", ++casos);
        if (B != 0) {
            int chamadas = contar_chamadas(N, B);
            while (chamadas >= B) chamadas %= B;
            printf("%d %d %d\n", N, B, chamadas);
        }
    } while (B != 0);

    return (0);
}
