#include <stdio.h>
#include <stdbool.h>
#include <locale.h>

float get_growth(int year_1, int year_1_usage, int year_2, int year_2_usage) {
	int years_no = year_2 - year_1;
	int usage_diff = year_2_usage - year_1_usage;
	return (usage_diff/(float)years_no);
}

int main () {
	int N;
	if (!scanf("%d", &N)) return -1;
	float solutions[N];

	for (int i = 0; i < N; i++) {
		int A, B, C, D;
		if (!scanf("%d %d %d %d", &A, &B, &C, &D)) return -1;

		solutions[i] = get_growth(A, B, C, D);		
	}
	for (int i = 0; i < N; i++) printf("%.2f\n", ((long int)(100 * solutions[i])) / 100.0);
	return 0;

}
