all: warmup
run: warmup
	./warmup

warmup: student.o main.o
	gcc -std=c11 -Wall -Wextra -o warmup student.o main.o

main.o: src/main.c src/student.h
	gcc -std=c11 -Wall -Wextra -c src/main.c

student.o: src/student.c src/student.h
	gcc -std=c11 -Wall -Wextra -c src/student.c

test: warmup
	diff <(cat tests/1.in | valgrind -q --leak-check=full ./warmup) tests/1.out
	diff <(cat tests/2.in | valgrind -q --leak-check=full ./warmup) tests/2.out
	diff <(cat tests/3.in | valgrind -q --leak-check=full ./warmup) tests/3.out
	diff <(cat tests/4.in | valgrind -q --leak-check=full ./warmup) tests/4.out
	diff <(cat tests/5.in | valgrind -q --leak-check=full ./warmup) tests/5.out
	diff <(cat tests/6.in | valgrind -q --leak-check=full ./warmup) tests/6.out
	diff <(cat tests/7.in | valgrind -q --leak-check=full ./warmup) tests/7.out
	diff <(cat tests/8.in | valgrind -q --leak-check=full ./warmup) tests/8.out
	diff <(cat tests/9.in | valgrind -q --leak-check=full ./warmup) tests/9.out

clean:
	rm *.o
