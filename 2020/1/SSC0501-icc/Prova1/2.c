#include <stdio.h>

typedef struct _tempo {
    int segundos;
    int minutos;
    int horas;
    int dias;
} tempo;


tempo converter_tempo(tempo entrada) {
    int dias = entrada.dias;
    int horas = entrada.horas + (24 * dias);
    int minutos = entrada.minutos + (60 * horas);
    int segundos = entrada.segundos + (60 * minutos);

    tempo saida;

    saida.dias = segundos/100000;
    segundos = segundos%10000;

    saida.horas = segundos/10000;
    segundos = segundos%10000;

    saida.minutos = segundos/100;
    segundos = segundos%100;

    saida.segundos = segundos;

    return(saida);
}

int main() {
    tempo entrada;
    printf("Digite dias: \n");
    scanf("%d", &entrada.dias);
    printf("Digite horas: \n");
    scanf("%d", &entrada.horas);
    printf("Digite minutos: \n");
    scanf("%d", &entrada.minutos);
    printf("Digite segundos: \n");
    scanf("%d", &entrada.segundos);

    tempo saida = converter_tempo(entrada);

    printf("Dias: %d, Horas: %d, Minutos: %d, Segundos: %d\n", saida.dias, saida.horas, saida.minutos, saida.segundos);

    return 0;
}


