#include <stdio.h>
#include <stdbool.h>

#define MAX_SIZE 10000

//Checks if provided numbers are multiples
int pages_number(int q, int d, int p) {
  int r = d*q*p / (p-q);
  return r;
}

int main() {
  int solution[MAX_SIZE]; //Solutions array (so we can print them all together)
  int n_cases; //Number of test cases (to be used when printing)
  int i;

  //Loop for each case
  for (i = 0; i < MAX_SIZE; i++) {
    int q, d, p;

    //Scanf is inside a for, to check if inputting was successful
    if (!scanf("%d", &q)) return -1;
    if (q == 0) break;
    if (!scanf("%d %d", &d, &p)) return -1;

    solution[i] = pages_number(q, d, p);
    n_cases = i;
  }
  //Print array until position "cases"
  for (i = 0; i <= n_cases; i++) {
    if (solution[i] != 1) {
      printf("%d paginas\n", solution[i]);
    }
    else {
      printf("%d pagina\n", solution[i]);
    }
  }

  return 0;
}
