#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *remover_letra(char *entrada, char letra) {
    char *saida = NULL;
    int tamanho_saida = 0;
    for(int i = 0; i < strlen(entrada); i++) {
        if (entrada[i] != letra) {
            tamanho_saida++;
            saida = realloc(saida, tamanho_saida*sizeof(char));
            saida[tamanho_saida-1] = entrada[i];
        }
    }
    saida = realloc(saida, (tamanho_saida+1)*sizeof(char));
    saida[tamanho_saida] = '\0';

    return (saida);
}

int main() {
    char *string_entrada;
    char char_entrada;

    printf("Digite a string: \n");
    scanf("%s", string_entrada);
    getchar();
    printf("Digite o caractere: \n");
    scanf("%c", &char_entrada);
    printf("%s\n", remover_letra(string_entrada, char_entrada));

    return 0;
}
