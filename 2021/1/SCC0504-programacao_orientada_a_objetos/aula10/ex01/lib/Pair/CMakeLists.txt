# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(Pair VERSION 1.0 DESCRIPTION "Par de tipos primitivos")

# Declarar biblioteca
add_library(Pair STATIC "src/pair.cpp")

# Adicionar includes
target_include_directories(Pair PUBLIC "include")
# Especificar c++ 17
set_property(TARGET Pair PROPERTY CXX_STANDARD 17)
