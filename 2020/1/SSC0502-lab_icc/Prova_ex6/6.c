#include <stdio.h>
#include <stdbool.h>

#define SIZE 1000

//Recebe string com os colchetes, checa se são válidos
bool colchetes_validos(char *in) {
    int abertos = 0;

    //Caso a string não esteja alocada
    if (in == NULL) return false;

    //Percorrer string até chegar no caractere nulo
    for (int i = 0; in[i] != '\0'; i++) {
        //Caso seja um colchete de fechamento
        if (in[i] == '}') {
            //Se não existe nenhum colchete aberto, inválido
            if (abertos < 1) {
                return false;
            }
            //Caso exista aberto, decrementar contagem
            else {
                abertos -= 1;
            }
        }
        //Caso seja um colchete de abertura, incrementar contagem
        else if (in[i] == '{') {
            abertos += 1;
        }
    }
    //Depois de percorrer tudo
    //Caso exista algum colchete ainda aberto, retornar inválido
    if (abertos != 0) {
        return false;
    }
    else {
        return true;
    }
}


int main() {
    char entrada[SIZE];
    //Ler do input
    fgets(entrada, SIZE, stdin);

    if (colchetes_validos(entrada)) printf("Correto\n");
    else printf ("Errado\n");
}
