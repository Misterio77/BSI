#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_SIZE 256

typedef struct _filme {
  char *titulo;
  char *genero;
  int ano;
} filme;

/*Aloca e retorna um filme*/
filme *criar_filme(char *titulo, char *genero, int ano) {
  filme *out = (filme*)malloc(sizeof(filme));
  out->titulo = strdup(titulo);
  out->genero = strdup(genero);
  out->ano = ano;

  return(out);
}

/*Desaloca completamente dado filme*/
void destruir_filme(filme *in) {
  free(in->titulo);
  free(in->genero);
  free(in);
}

/*Receber do input do usuário a quantidade especificada de filmes*/
filme **receber_filmes(int tamanho) {
  char buffer[MAX_STRING_SIZE];

  filme **vetor_filmes = (filme**)malloc(sizeof(filme*) * tamanho);
  if(vetor_filmes == NULL) return(NULL); //Verificar se foi possível alocar

  for(int i = 0; i < tamanho; i++) {
    //Variáveis temporárias para ler
    char in_titulo[MAX_STRING_SIZE], in_genero[MAX_STRING_SIZE];
    int in_ano;

    printf("Digite o título, gênero e ano do filme %d (Apertando enter a cada campo): \n", i+1);

    if(
      //Ler e formatar titulo
      fgets(buffer, MAX_STRING_SIZE, stdin) != NULL ||
      sscanf(buffer, "%[^\n]", in_titulo) != 1      ||
      //Ler e formatar genero
      fgets(buffer, MAX_STRING_SIZE, stdin) != NULL ||
      sscanf(buffer, "%[^\n]", in_genero) != 1      ||
      //Ler e formatar ano
      fgets(buffer, MAX_STRING_SIZE, stdin) != NULL ||
      sscanf(buffer, "%d", &in_ano) != 1
    ) {
      //Caso algo de errado aconteça, destruir o vetor e retornar NULL
      for(int j = 0; j < i; j++) destruir_filme(vetor_filmes[j]);
      free(vetor_filmes);
      return(NULL);
    }
    printf("\n");
    vetor_filmes[i] = criar_filme(in_titulo, in_genero, in_ano);

  }

  return(vetor_filmes);
}

/*Dado filme, imprime as informações*/
void print_filme(filme *in) {
  printf("%s | Gênero: %s | Ano: %d\n", in->titulo, in->genero, in->ano);
}

/*Dado vetor de filmes, seu tamanho, imprime os filmes produzidos no intervalo de anos especificado*/
void print_filmes_ano(filme **filmes, int n_filmes, int primeiro_ano, int ultimo_ano) {
  for(int i = 0; i < n_filmes; i++) {
    if(filmes[i]->ano >= primeiro_ano && filmes[i]->ano <= ultimo_ano) {
      print_filme(filmes[i]);
    }
  }
}

/*Dado vetor de filmes, seu tamanho e um genero, imprime os filmes com aquele genero*/
void print_filmes_genero(filme **filmes, int n_filmes, char *genero) {
  for(int i = 0; i < n_filmes; i++) {
    if(strcmp(filmes[i]->genero, genero) == 0) {
      print_filme(filmes[i]);
    }
  }
}

int main() {
  filme **A; //Criar o ponteiro que irá apontar para o vetor de filmes
  A = receber_filmes(5);
  if(A == NULL) {
    printf("**Erro ao ler o vetor**\n");
    free(A);
    return(-1);
  }

  printf("Filmes produzidos entre 2001 e 2005:\n");
  print_filmes_ano(A, 5, 2001, 2005);

  printf("Filmes do genero Terror:\n");
  print_filmes_genero(A, 5, "Terror");

  //Liberar  cada filme do vetor
  for(int i = 0; i < 5; i++) destruir_filme(A[i]);
  //Liberar vetor filmes
  free(A);

  return(0);
}
