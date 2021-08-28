#include <stdio.h>
#include <stdlib.h>

/*Calcula o valor de um elemento do triangulo de pascal, dado o vetor da linha anterior, sua linha (n) e coluna (k)*/
unsigned long long int elemento_triang_pascal(unsigned long long int *linha_anterior, int n, int k) {
    //Vamos retornar 1 se a linha anterior não existir ou se for o primeiro ou ultimo numero da linha
    if (linha_anterior == NULL || n <= 0 || k <= 0 || k >= n) return 1;
    else {
        //Somar pai esquerdo e pai direito
        return (linha_anterior[k] + linha_anterior[k-1]);
    }
}

/*Dado linha anterior (pode ser null) e numero da linha, aloca e retorna um vetor com todos os elementos da linha especificada*/
unsigned long long int *linha_triang_pascal(unsigned long long int *anterior, int n) {
    //Alocar vetor e checar por erros
    unsigned long long int *retorno = malloc(sizeof(unsigned long long int) * (n+1));
    if (retorno == NULL) {
        printf("** Erro ao alocar **\n");
        return NULL;
    }

    //Calcular elemento por elemento
    for (int i = 0; i <= n; i++) {
        retorno[i] = elemento_triang_pascal(anterior, n, i);
    }
    //Retornar o vetor preenchido
    return retorno;
}

/*Imprime um vetor de unsigned long long int, dado seu tamanho*/
void imprimir_vetor(unsigned long long int *vetor, int tam) {
    for (int i = 0; i < tam; i++) printf("%llu ", vetor[i]);
    printf("\n");
}


int main() {
    int n;
    printf("Digite o tamanho do triângulo\n");
    printf("(Funciona para casos bem grandes)\n");
    printf("> ");
    scanf("%d", &n);

    //Vamos armazenar dois vetores por vez
    unsigned long long int *linha = NULL;
    unsigned long long int *linha_anterior = NULL;

    for (int i = 0; i <= n; i++) {
        //Calcular a linha atual
        linha = linha_triang_pascal(linha_anterior, i);
        //Sair do programa caso ocorra erro ao alocar
        if (linha == NULL) return(-1);
        //E imprimí-la
        imprimir_vetor(linha, i+1);

        //Caso não seja o primeiro caso, liberar a linha anterior da memoria
        if (i > 0) free(linha_anterior);
        //Mover a linha atual para anterior
        linha_anterior = linha;
        //Colocar NULL na linha atual (proxima a ser lida)
        linha = NULL;
    }
    //Liberar a ultima linha antes de sair
    free(linha_anterior);

    return (0);
}
