#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>
#include <locale.h>

#define MAX_SIZE 10000

double yearly_growth(int a, long long int b, int c, long long int d) {
    int length = c-a;
    long long int change = d-b;

    return change/(double)length;
}

int main() {
    double solution[MAX_SIZE]; 
    int n, i;
    if (!scanf("%d", &n)) return(-1);

    for (i = 0; i < n; i++) {
        int a, c;
        long long int b, d;

        if (!scanf("%d %lld %d %lld", &a, &b, &c, &d)) return(-1);
        solution[i] = ((long long int)(yearly_growth(a, b, c, d) * 100)) / 100.0;
    }

    for (i = 0; i < n; i++) {
        int intpart = (int)solution[i];
        int decpart = (solution[i] - intpart)*1000;
        if (decpart != 0 && ((decpart % 10)%10) > 5) decpart+= 10;
        printf("%d,%d", intpart, decpart/10);
        if (decpart == 0) printf("0");
        printf("\n");
    }

    return(0);
}
