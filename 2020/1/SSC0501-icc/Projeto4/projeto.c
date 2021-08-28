#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define MAX_STRING_SIZE 256

typedef struct _filme {
  char *titulo;
  char *genero;
  int ano;
  int num_catalogo;
} filme;

/*Aloca e retorna um filme*/
filme *criar_filme(char *titulo, char *genero, int ano, int num_catalogo) {
  filme *out = (filme*)malloc(sizeof(filme));
  out->titulo = strdup(titulo);
  out->genero = strdup(genero);
  out->ano = ano;
  out->num_catalogo = num_catalogo;

  return(out);
}

/*Desaloca completamente dado filme*/
void destruir_filme(filme *in) {
  free(in->titulo);
  free(in->genero);
  free(in);
}

/*Adiciona um filme especificado ao vetor, retorna o novo tamanho dele (0 se der algum erro)*/
filme **post_filmes(filme **vetor, int *n_filmes, filme *in) {
  if (in == NULL) return(NULL); //Verificar se o elemento a ser adicionado está alocado

  vetor = (filme**)realloc(vetor, sizeof(filme*) * ((*n_filmes)+1)); //Crescer vetor
  if(vetor == NULL) return(NULL); //Verificar se foi possível realocar

  vetor[*n_filmes] = in;

  *n_filmes += 1;

  return(vetor);
}

/*Busca por um filme (usando numero do catalogo) e o substitui por dado filme. Retorna 0 se falhar*/
filme **put_filmes(filme **vetor, int *n_filmes, int num_catalogo, filme *in) {
  int pos;
  for (pos = 0; pos < *n_filmes; pos++) {
    if (vetor[pos]->num_catalogo == num_catalogo) {
      destruir_filme(vetor[pos]);
      vetor[pos] = in;
      return(vetor);
    }
  }
  vetor = post_filmes(vetor, n_filmes, in);
  return(vetor);
}

/*Busca por um filme (usando numero do catalogo), o remove do vetor (fazendo shift e desalocando a ultima posicao), retorna o novo tamanho (0 se falhar)*/
filme **del_filmes(filme **vetor, int *n_filmes, int num_catalogo) {
  for (int pos = 0; pos < *n_filmes; pos++) {
    //Caso exista pelo menos um a ser deletado
    if (vetor[pos]->num_catalogo == num_catalogo) {
      //Vetor auxiliar
      filme **vetor_aux;

      //Vamos destruir o filme a ser apagado, e copiar todos os outros
      //Iterador para o vetor auxiliar
      int j = 0;
      //Percorrer o vetor original
      for (int i = 0; i < *n_filmes; i++) {
        //Caso seja o alvo de remoção
        if (vetor[i]->num_catalogo == num_catalogo) {
          destruir_filme(vetor[i]);
        }
        //Caso contrario, copiar pro auxiliar
        else {
          //Realocar o vetor auxiliar para o tamanho nescessario até agora
          vetor_aux = realloc(vetor_aux, sizeof(filme*) * j+1);
          vetor_aux[j] = vetor[i];
          j++;
        }
      }
      //Agora vamos apagar o vetor antigo, e o substituir por aux
      free(vetor);
      vetor = vetor_aux;
      *n_filmes = j;
      //Retornar o novo vetor
      return(vetor);
    }
  }
  printf("** Filme não encontrado **\n");
  return(vetor);
}

/*Receber do input do usuário um filme, alocar e retorná-lo*/
filme *receber_filme() {
  char buffer[MAX_STRING_SIZE];

  char in_titulo[MAX_STRING_SIZE], in_genero[MAX_STRING_SIZE];
  int in_ano, in_num_catalogo;

  printf("Digite o título, gênero, ano e numero de catalogo do filme (Apertando enter a cada campo): \n");

  if(
    //Ler e formatar titulo
    fgets(buffer, MAX_STRING_SIZE, stdin) == NULL ||
    sscanf(buffer, "%[^\n]", in_titulo) != 1      ||
    //Ler e formatar genero
    fgets(buffer, MAX_STRING_SIZE, stdin) == NULL ||
    sscanf(buffer, "%[^\n]", in_genero) != 1      ||
    //Ler e formatar ano
    fgets(buffer, MAX_STRING_SIZE, stdin) == NULL ||
    sscanf(buffer, "%d", &in_ano) != 1            ||
    //Ler e formatar num_catalogo
    fgets(buffer, MAX_STRING_SIZE, stdin) == NULL ||
    sscanf(buffer, "%d", &in_num_catalogo) != 1
  ) {
    printf("** Erro ao ler o filme **\n");
    return(NULL);
  }
  filme *novo_filme = criar_filme(in_titulo, in_genero, in_ano, in_num_catalogo);
  return(novo_filme);
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
  filme **A = NULL; //Criar o ponteiro que irá apontar para o vetor de filmes
  int A_tamanho = 0;

  for (int i = 0; i < 2; i++) {
    A = post_filmes(A, &A_tamanho, receber_filme());
    printf("tamanho: %d\n", A_tamanho);
  }

  A = del_filmes(A, &A_tamanho, 123);
  printf("tamanho: %d\n", A_tamanho);
  A = post_filmes(A, &A_tamanho, receber_filme());
  printf("tamanho: %d\n", A_tamanho);

  for (int i = 0; i < A_tamanho; i++) {
    destruir_filme(A[i]);
    free(A);
  }
  return(0);
}
