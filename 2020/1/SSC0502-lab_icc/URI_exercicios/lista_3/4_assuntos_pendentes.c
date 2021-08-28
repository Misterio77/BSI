#include <stdio.h>

int esta_aberto(char *in) {
    int abertos = 0;
    int i;
    for(i = 0; in[i] != '\0'; i++) {
        if (in[i] == '(') {
            abertos += 1;
        }
        else if (in[i] == ')') {
            if (abertos > 0) abertos -= 1;
        }
    }
    return abertos;
}

int main() {
    char entrada[100000];
    scanf("%s", entrada);

    int abertos = esta_aberto(entrada);
    if (abertos == 0) printf("Partiu RU!\n");
    else printf("Ainda temos %d assunto(s) pendentes!\n", abertos);

    return 0;
}


