#ifndef STUDENT_H_
#define STUDENT_H_

#include<stdio.h>

#define FIELD_LEN 50
#define STUDENT_SIZE (sizeof(int) + (sizeof(char)*FIELD_LEN) + (sizeof(char)*FIELD_LEN) + (sizeof(char)*FIELD_LEN) + sizeof(float))

// Representa um estudante
typedef struct student {
    int number;
    char first_name[FIELD_LEN];
    char last_name[FIELD_LEN];
    char major[FIELD_LEN];
    float score;
} Student;

Student *student_from_file(FILE *input);
int student_to_file(Student *in, FILE *out);
Student *student_from_csv(const char *input);
void student_display(Student *input);
void student_destroy(Student *input);
unsigned long student_size();

#endif
