# Versão do CMake
cmake_minimum_required(VERSION 3.10)
# Especificação de comandos em json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


# Info do projeto
project(Devices VERSION 1.0 DESCRIPTION "Dispositivos (Rádio, Relógio, Rádio-Relógio)")

# Declarar biblioteca
add_library(Devices STATIC "src/devices.cpp")

# Adicionar includes
target_include_directories(Devices PUBLIC "include")
# Especificar c++ 17
set_property(TARGET Devices PROPERTY CXX_STANDARD 17)
