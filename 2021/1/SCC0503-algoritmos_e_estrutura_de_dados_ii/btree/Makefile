NAME := ex3
OUT_DIR := build
SRC_DIR := src
FILES := btree.o database.o main.o student.o utils.o
OBJS := $(foreach file,$(FILES),$(OUT_DIR)/$(file))

all: $(NAME)

run: $(NAME)
	./$(NAME)

$(NAME): $(OBJS)
	gcc -std=c11 -g -Wall -Wextra -o $(NAME) $(OBJS)

$(OBJS): $(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(OUT_DIR)
	gcc -std=c11 -g -Wall -Wextra -c $< -o $@

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

clean:
	rm $(OBJS) $(NAME)

reset:
	rm *.dat
