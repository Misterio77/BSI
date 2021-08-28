#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

//Estrutura para coordenadas
typedef struct _coordenadas {
    int maior_linha;
    int maior_coluna;
    int menor_linha;
    int menor_coluna;
} coordenadas;

coordenadas maior_e_menor_coordenadas(int **matriz, int linhas, int colunas) {
    //Temporariamente guardam o maior e o menor numeros encontrados ate o momento
    int maior = INT_MIN, menor = INT_MAX;
    coordenadas retorno;
    for (int i = 0; i < linhas; i++) {
        for (int j = 0; j < colunas; j++) {
            if (matriz[i][j] > maior) {
                retorno.maior_linha = i;
                retorno.maior_coluna = j;
                maior = matriz[i][j];
            }
            if (matriz[i][j] < menor) {
                retorno.menor_linha = i;
                retorno.menor_coluna = j;
                menor = matriz[i][j];
            }
        }
    }
    return (retorno);
}
