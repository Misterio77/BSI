# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(lista05-ex02 VERSION 1.0 DESCRIPTION "Exercício 2 da Lista 5")

# Adicionar biblioteca
add_subdirectory(lib/ComplexNumber)
# Adicionar executável
add_executable(lista05-ex02 src/main.cpp)

# Linkar executavel com a biblioteca
target_link_libraries(lista05-ex02 ComplexNumber)
# Especificar c++ 17
set_property(TARGET lista05-ex02 PROPERTY CXX_STANDARD 17)
