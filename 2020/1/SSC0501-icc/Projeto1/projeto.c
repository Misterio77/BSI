#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

//Macro para comando de limpar a tela e esperar tempo.
#ifdef _WIN32 //Caso do windows, usaremos 'cls' e incluiremos windows.h
#define CLEAR "cls"
#include <Windows.h>
#define sleep(x) Sleep(1000*x)
#else //Em outros casos, vamos assumir que é Unix-like e usar 'clear' e pegar unistd.h
#define CLEAR "clear"
#include <unistd.h>
#endif


//Adiciona um numero aleatório (com alcance especificado, nao inclusivo) dado um array (e seu novo tamanho)
int *adicionar_aleatorio(int *array, int len, int range) {
  int aleatorio = rand()%range;
  //Expandir o array (ou criar, caso nescessario)
  if (len == 1) {
    array = malloc(len * sizeof(int));
  }
  else {
    array = realloc(array, (len * sizeof(int)));
  }
  //Adicionar o numero aleatorios
  array[len-1] = aleatorio;

  return array;
}

//Verifica se dois arrays (com o mesmo tamanho) sao iguais
bool comparar_arrays(int *a, int *b, int len) {
  for(int i = 0; i < len; i++) {
    if(a[i] != b[i]) return(false);
  }

  return(true);
}

void printar_array(int *array, int len) {
  for(int i = 0; i < len; i++) {
    printf("%d ", array[i]);
  }
}

int *ler_array(int len) {
  int *array = malloc(len * sizeof(int));
  for(int i = 0; i < len; i++) {
    scanf("%1d", &array[i]);
  }

  return(array);
}

//Função que executa uma partida do jogo, separei do main para que o escopo das variáveis fique mais organizado
int jogo() {
  //Ponteiro que usaremos para armazenar o array de numeros durante a execução, inicialmente vazio.
  int *numeros;

  for(int i = 1 ;; i++) {
    //Adicionar numero aleatorio no array
    numeros = adicionar_aleatorio(numeros, i, 10);

    //Mostrar sequencia para o usuario por 5 segundos
    printf("Sequencia para memorizar: ");
    printar_array(numeros, i);
    printf("\n");

    sleep(5);

    system(CLEAR);

    //Ler e comparar a entrada, depois liberar da memória
    printf("Digitar sequencia: ");
    int *entrada = ler_array(i);
    bool acertou = comparar_arrays(entrada, numeros, i);
    free(entrada);

    if (!acertou) {
      free(numeros);
      return(i-1); //Placar
    }
    printf("Resposta correta!\n\n");
  }
}


int main() {
  //Inicializar srand
  srand(time(NULL));

  int placar = jogo();
  printf("Fim do jogo! Voce fez %d ponto(s).\n", placar);
  getchar();
  getchar();

  return(0);
}
