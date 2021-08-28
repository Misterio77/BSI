#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

/*
Para evitar ter que manter o tamanho do vetor de strings guardado, eu simplesmente coloco um NULL numa posição adicional.
Essa função itera até NULL e devolve o tamanho do vetor (sem contar o NULL).
Caso o vetor dado seja inválido, simplesmente retorna 0.
*/
int tam_vetor(char **in) {
    if (in == NULL) return(0);

    int i;
    for (i = 0; in[i] != NULL; i++);
    
    return(i);
}
/*
Libera toda a memoria de um vetor de strings
*/
void liberar_vetor(char **in) {
    for (int i = 0; i < tam_vetor(in); i++) {
        free(in[i]);
    }
    free(in);
}


/*
Imprimir um vetor de strings. Não imprime nada caso o vetor seja inválido.
*/
void print_vetor(char **in) {
    if (in == NULL) return;

    for (int i = 0; i < tam_vetor(in); i++) {
        printf("%s ", in[i]);
    }
    printf("\n");
}

/*
Dado um ponteiro para arquivo, retorna um vetor com todas as palavras escritas nele (Separadas por espaço ou quebra de linha).
Retorna NULL caso dê algum erro, inclusive se os arquivos estiverem vazios.
 */
char **pegar_palavras(FILE *in) {
    char tmp[2048]; //Buffer estatico
    char **out = NULL;
    int tamanho = 0; //Numero de palavras no vetor

    //Enquanto n encontrarmos EOF
    while(fgets(tmp, 2048, in)) {
        tmp[strcspn(tmp, "\n")] = 0; //Remover newline no fim
        char *palavra;
        const char *divisor = " ";
        
        //strtok inicial
        palavra = strtok(tmp, divisor);
        while(palavra != NULL) {
            //Realocar, aumentando em 1 o tamanho do vetor
            out = (char **) realloc(out, sizeof(char *)*(++tamanho));
            //Colocar a palavra na nova posicao
            out[tamanho-1] = strdup(palavra);
            //Pegar proxima palavra
            palavra = strtok(NULL, divisor);
        }
        free(palavra);
    }
    //Caso alguma palavra tenha sido colocada
    if (tamanho > 0) {
        //Criar uma nova posição no final
        out = (char **) realloc(out, sizeof(char *)*(tamanho+1));
        //Vamos colocar NULL na ultima posição, para ser possível iterar
        out[tamanho] = NULL;
    }
    //Retornar vetor
    return (out);

}
/*
Dado dois vetores de palavras, retorna um vetor com as palavras presentes em ambos (duplicados aparecem apenas 1 vez).
Retorna NULL caso não haja nenhum.
*/
char **palavras_em_ambos(char **vetor1, char **vetor2) {
    char **out = NULL;
    int tamanho = 0; //Tamanho do vetor saida

    for (int i = 0; i < tam_vetor(vetor1); i++) { //Percorrer vetor1
        for (int j = 0; j < tam_vetor(vetor2); j++) { //Percorrer vetor2
            if (strcmp(vetor1[i], vetor2[j]) == 0) { //Caso encontremos uma correspondencia nos dois vetores
                //Vamos verificar se no vetor saida ja tem, para evitar duplicados
                bool existe = false;
                for (int k = 0; k < tamanho; k++) {
                    if (strcmp(vetor1[i], out[k]) == 0) {//Caso ja exista
                        existe = true;
                    }
                }
                if (!existe) { //Vamos adicionar a palavra no vetor saida
                    //Crescer vetor
                    out = (char **) realloc(out, sizeof(char *)*(++tamanho));
                    out[tamanho-1] = strdup(vetor1[i]);
                }
            }
        }
    }
    //Caso alguma palavra tenha sido colocada
    if (tamanho > 0) {
        //Aumentar o tamanho em 1
        out = (char **) realloc(out, sizeof(char *)*(tamanho+1));
        //Vamos colocar NULL na ultima posição, para ser possível iterar
        out[tamanho] = NULL;
    }
    //Retornar vetor
    return(out);
}

int main(void) {
    //Nomes dos seus arquivos
    const char *caminho1 = "1.txt";
    const char *caminho2 = "2.txt";
    
    //Abrir arquivos
    FILE *arquivo1 = fopen(caminho1, "r");
    FILE *arquivo2 = fopen(caminho2, "r");
    //Checar por erros ao abrir
    if (arquivo1 == NULL || arquivo2 == NULL) {
        printf("Erro ao abrir os arquivos! (Eles existem?)\n");
        return(-1);
    }

    //Extrair as palavras dos arquivos
    //Não há nescessidade de checar se retornaram NULL, a função de tamanho (e por consequência as de imprimir e comparar) lida com casos de texto vazio sem erros de runtime.
    char **palavras1 = pegar_palavras(arquivo1);
    char **palavras2 = pegar_palavras(arquivo2);
    fclose(arquivo1);
    fclose(arquivo2);
    
    //Procurar correspondência de palavras
    char **correspondencias = palavras_em_ambos(palavras1, palavras2);
    print_vetor(correspondencias);
    
    //Finalmente, retornar os vetores
    liberar_vetor(palavras1);
    liberar_vetor(palavras2);
    liberar_vetor(correspondencias);

    return(0);
}
