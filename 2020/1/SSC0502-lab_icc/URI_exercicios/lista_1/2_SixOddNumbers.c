#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

//Checks if given int is an even number
bool is_even(int in) {
  if(in % 2 == 0) {
    return true;
  }
  else {
    return false;
  }
}

//Returns an int array with the next six odd numbers from a provided int
int *get_odd_numbers(int in) {
  int *r = malloc(6 * sizeof(int));
  int i;

  for (i = 0; i < 6; i++) {
    int offset = 0; //We'll start from the number itself if it's odd.
    if(is_even(in)) offset = 1; //If it's even, offset by one.

    r[i] = in + offset + i*2;
  }
  return r;
}

int main() {
  int input, i;
  //Scanf is inside a for, to check if inputting was successful
  if (scanf("%d", &input)) {
    int *output = get_odd_numbers(input);
    for(i = 0; i < 6; i++) printf("%d\n", output[i]);

    free(output);
    return 0;
  }
  else {
    return -1;
  }
}
