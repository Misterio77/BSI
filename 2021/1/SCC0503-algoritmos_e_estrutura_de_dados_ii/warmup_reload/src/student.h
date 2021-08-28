#ifndef STUDENT_H_
#define STUDENT_H_

#include<stdio.h>

typedef struct _student student;
student *student_from_file(FILE *input);
int student_to_file(student *in, FILE *out);
student *student_from_csv(char *input);
void student_display(student *input);
void student_destroy(student *input);
unsigned long student_size();

#endif
