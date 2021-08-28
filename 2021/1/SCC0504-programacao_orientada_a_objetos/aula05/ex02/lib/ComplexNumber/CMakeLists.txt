# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(ComplexNumber VERSION 1.0 DESCRIPTION "Números Complexos")

# Declarar biblioteca
add_library(ComplexNumber STATIC "src/complex_number.cpp")

# Adicionar includes
target_include_directories(ComplexNumber PUBLIC "include")
# Especificar c++ 17
set_property(TARGET ComplexNumber PROPERTY CXX_STANDARD 17)
