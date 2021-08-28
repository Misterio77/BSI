# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(Stack VERSION 1.0 DESCRIPTION "Pilha")

# Declarar biblioteca
add_library(Stack STATIC "src/stack.cpp")

# Adicionar includes
target_include_directories(Stack PUBLIC "include")
# Especificar c++ 17
set_property(TARGET Stack PROPERTY CXX_STANDARD 17)
