#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#include <unistd.h>

#define MAX_SIZE 10000

//Given desired building area, and max build percentage, return (truncated) plot dimension (assuming plot is square-shaped).
int get_plot_size(int build_area, int max_percentage) {
  double plot_area = build_area * (100 / (double)max_percentage);
  int plot_dim = (int)sqrt(plot_area);

  return plot_dim;
}



int main() {
  int solution[MAX_SIZE]; //Solutions array (so we can print them all together)
  int n_cases = 0; //Number of test cases (to be used when printing)
  int i;
  //Loop for each case
  for (i = 0; i < MAX_SIZE; i++) {
    int a, b, c;

    //Receive the 3 values
    //(All scanf returns are checked for best practices (and avoid -O2 compilation complaining))
    if (!scanf("%d", &a)) return -1;
    if (a == 0) break; //Exit loop if a is 0
    if (!scanf("%d %d", &b, &c)) return -1;

    //Store the solution
    solution[i] = get_plot_size(a*b, c);

    //Number of cases
    n_cases = i;
  }
  //Print array until position "cases"
  for (i = 0; i <= n_cases; i++) printf("%d\n", solution[i]);

  return 0;
}
