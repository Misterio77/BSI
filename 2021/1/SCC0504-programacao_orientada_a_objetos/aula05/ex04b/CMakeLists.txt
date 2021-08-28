# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(lista05-ex04b VERSION 1.0 DESCRIPTION "Exercício 4 B da Lista 5")

# Adicionar biblioteca
add_subdirectory(lib/Devices)
# Adicionar executável
add_executable(lista05-ex04b src/main.cpp)

# Linkar executavel com a biblioteca
target_link_libraries(lista05-ex04b Devices)
# Especificar c++ 17
set_property(TARGET lista05-ex04b PROPERTY CXX_STANDARD 17)
