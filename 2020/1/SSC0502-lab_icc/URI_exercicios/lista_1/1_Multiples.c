#include <stdio.h>
#include <stdbool.h>

//Checks if provided numbers are multiples
bool is_multiple (int a, int b){
  if (
    a % b == 0 ||
    b % a == 0
  ) {
    return true;
  }
  else {
    return false;
  }
}

int main() {
  int input_a, input_b;
  //Scanf is inside a for, to check if inputting was successful
  if (!scanf("%d %d", &input_a, &input_b)) return -1;

  if(is_multiple(input_a, input_b)) printf("Sao Multiplos\n");
  else                              printf("Nao sao Multiplos\n");

  return 0;
}
