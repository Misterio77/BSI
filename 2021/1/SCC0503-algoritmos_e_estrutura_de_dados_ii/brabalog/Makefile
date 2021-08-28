# Wildcard recursivo pra pegar as .java em vários níveis
# (https://stackoverflow.com/questions/4036191/sources-from-subdirectories-in-makefile)
rwildcard=$(wildcard $1$2) $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2))

# Diretório com os arquivos fonte
SRC_DIR := src/main/java

# Diretório para os artefatos compilados
OUT_DIR := target
# Diretório com as classes construídas
CLS_DIR := $(OUT_DIR)/classes

# Pegar todas as .java dentro de SRC_DIR
SRCS := $(call rwildcard,$(SRC_DIR),*.java)

# Lista de todas as classes
# exemplo: src/main/java/me/misterio/alg2brabalog/Main.java -> target/classes/me/misterio/alg2brabalog/Main.class
CLS := $(SRCS:$(SRC_DIR)/%.java=$(CLS_DIR)/%.class)

# Flags de compilação
JCFLAGS := -d $(OUT_DIR)/ -cp $(CLS_DIR)/

# Caminho para a classe main
MAIN := me.misterio.alg2brabalog.Main

# Nome da.jar
JAR := $(OUT_DIR)/alg2brabalog-1.0.jar


# Diretriz padrão (compilar a .jar e tds os requisitos)
all: $(JAR)

# Criar jar, especificando a main e as classes (com o target/classes removido da Main.class)
$(JAR): $(CLS)
	$(info CLS is $(CLS))
	jar --create --file $@ --main-class $(MAIN) -C $(CLS_DIR) $(CLS:$(CLS_DIR)%/Main.class=%/Main.class)

# Compilar classes
# -d indica qual diretório colocar
# -cp é o caminho das classes
$(CLS): $(CLS_DIR)/%.class: $(SRC_DIR)/%.java
	javac $< -d $(CLS_DIR) -cp $(SRC_DIR)

# Retirar todos os artefatos compilados
clean:
	rm $(OUT_DIR) -r

# Executar a jar
run: $(JAR)
	@java -jar $(JAR)
