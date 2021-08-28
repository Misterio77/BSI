# Lista 5
Cada exercício é estruturado como uma biblioteca + main que a consome.

Em cada exercício, a main fica em `src/main.cpp`, e os outros componentes ficam em `lib/NomeDoComponente/src/arquivo.cpp`, com seus respectivos headers (hpp) em `lib/NomeDoComponente/include/arquivo.hpp`.

## Como compilar
Basta usar o CMake. Caso você não tenha instalado, é só usar (no Ubuntu): `sudo apt install cmake`

Entre no diretório do exercício que você quer compilar. Daí entre na pasta build (pode criar, caso não exista), e use:

`cmake ..`

`cmake --build .`


## Como executar
O executável com o nome correspondente (por exemplo, lista05-ex02) aparecerá dentro da pasta. Basta e usar `./lista06-ex02` (por exemplo) para executar o programa.
