all: warmup_reload
run: warmup_reload
	./warmup_reload

warmup_reload: student.o main.o
	gcc -std=c11 -g -Wall -Wextra -o warmup_reload student.o main.o

main.o: src/main.c src/student.h
	gcc -std=c11 -g -Wall -Wextra -c src/main.c

student.o: src/student.c src/student.h
	gcc -std=c11 -g -Wall -Wextra -c src/student.c

test: warmup_reload
	diff <(cat tests/1.in | valgrind -q --leak-check=full ./warmup_reload) tests/1.out
	diff <(cat tests/2.in | valgrind -q --leak-check=full ./warmup_reload) tests/2.out

clean:
	rm *.o
