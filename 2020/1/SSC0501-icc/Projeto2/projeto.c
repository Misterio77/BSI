#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

//Aloca e devolve o ponteiro de uma matriz, dadas suas dimensões
int **alloc_matrix(int rows, int cols) {
  if (rows < 1 || cols < 1) {
    return NULL;
    printf("** Error: invalid size **\n");
  }

  int **matrix = (int **) calloc(rows, sizeof(int *));
  if (matrix == NULL) return NULL;

  for (int i = 0; i < rows; i++) {
    matrix[i] = (int *) calloc(cols, sizeof(int));
    if (matrix[i] == NULL) return NULL;
  }

  return matrix;
}

//Libera uma matriz da memoria
int **free_matrix(int **matrix, int rows, int cols) {
  if (matrix == NULL) return NULL;
  if (rows < 1 || cols < 1) {
    return matrix;
    printf("** Error: invalid size **\n");
  }

  for (int i = 0; i < rows; i++) free(matrix[i]);
  free(matrix);
  return NULL;
}

//Recebe duas matrizes (e suas dimensões), e retorna a matriz soma.
int **matrix_sum(int **A, int A_rows, int A_cols, int **B, int B_rows, int B_cols) {
  int **result;
  int result_rows, result_cols;

  //Vamos pegar o maximo entre as dimensoes de A e B
  if (A_rows > B_rows) result_rows = A_rows;
  else                 result_rows = B_rows;
  if (A_cols > B_cols) result_cols = A_cols;
  else                 result_cols = B_cols;

  //Alocar a matriz soma
  result = alloc_matrix(result_rows, result_cols);
  if (result == NULL) return NULL;
  //Iterar pelos elementos
  for (int current_row = 0; current_row < result_rows; current_row++) {
    for (int current_col = 0; current_col < result_cols; current_col++) {
      //Caso a posição atual ultrapasse uma (ou ambas) dimensão da matriz B, vamos colocar 0 ao invés de tentar acessar a matriz.
      int A_value = ((current_row >= A_rows) || (current_col >= A_cols)) ? 0 : A[current_row][current_col];
      //O mesmo para a matriz B
      int B_value = ((current_row >= B_rows) || (current_col >= B_cols)) ? 0 : B[current_row][current_col];
      result[current_row][current_col] = A_value + B_value;
    }
  }

  return result;
}

//Imprime uma matriz na tela, separando colunas por espaços e fileiras por quebra de linha
//Tambem temos a opcao de espaçamento, que aqui eu só implementei para numeros de no máximo 5 digitos
void print_matrix(int **matrix, int rows, int cols, bool evenly_spaced) {
  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      if (evenly_spaced) {
        if (matrix[i][j] < 10000) printf(" ");
        if (matrix[i][j] < 1000) printf(" ");
        if (matrix[i][j] < 100) printf(" ");
        if (matrix[i][j] < 10) printf(" ");
      }
      printf("%d ", matrix[i][j]);
    }
    printf("\n");
  }
}

//Gera uma matriz de inteiros aleatorios, dado tamanho e range maximo
int **random_matrix(int rows, int cols, int max_range) {
  int **matrix = alloc_matrix(rows, cols);
  if (matrix == NULL) return NULL;

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      matrix[i][j] = rand()%max_range;
    }
  }

  return matrix;
}

int main(int argc, char const *argv[]) {
  srand(time(NULL));

  //Não entendi muito bem se era pro usuario digitar as matrizes também, além das dimensões
  //Então pensei em fazer um gerador aleatório de matriz, para o tamanho especificado
  int A_rows, A_cols, B_rows, B_cols;

  printf("Digite o numero de linhas e colunas para a primeira matriz \n>");
  scanf("%d %d", &A_rows, &A_cols);
  int **A = random_matrix(A_rows, A_cols, 100);
  printf("Sua primeira matriz:\n");
  print_matrix(A, A_rows, A_cols, true);
  printf("\n");

  printf("Digite o numero de linhas e colunas para a segunda matriz \n>");
  scanf("%d %d", &B_rows, &B_cols);
  int **B = random_matrix(B_rows, B_cols, 100);
  printf("Sua segunda matriz:\n");
  print_matrix(B, B_rows, B_cols, true);
  printf("\n");


  int **C = matrix_sum(A, A_rows, A_cols, B, B_rows, B_cols);
  int C_rows, C_cols;
  //Pegar o valor maximo entre as dimensoes, para a matriz soma
  if (A_rows > B_rows) C_rows = A_rows;
  else                 C_rows = B_rows;
  if (A_cols > B_cols) C_cols = A_cols;
  else                 C_cols = B_cols;

  printf("Matriz soma: \n");
  print_matrix(C, C_rows, C_cols, true);
  printf("\n");

  if (free_matrix(A, A_rows, A_cols) != NULL) return (1);
  if (free_matrix(B, B_rows, B_cols) != NULL) return (1);
  if (free_matrix(C, C_rows, C_cols) != NULL) return (1);

  return (0);
}
