# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(lista10-ex02 VERSION 1.0 DESCRIPTION "Exercício 2 da Lista 10")

# Adicionar biblioteca
add_subdirectory(lib/Stack)
# Adicionar executável
add_executable(lista10-ex02 src/main.cpp)

# Linkar executavel com a biblioteca
target_link_libraries(lista10-ex02 Stack)
# Especificar c++ 17
set_property(TARGET lista10-ex02 PROPERTY CXX_STANDARD 17)
