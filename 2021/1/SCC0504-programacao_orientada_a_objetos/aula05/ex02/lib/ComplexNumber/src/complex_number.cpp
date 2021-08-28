#include <ostream>
#include "complex_number.hpp"

// Construtor
ComplexNumber::ComplexNumber(float real, float imag) {
    r = real;
    i = imag;
}

// Soma
ComplexNumber ComplexNumber::sum(ComplexNumber const& other) {
    // Somar cada parte
    return ComplexNumber(this->r+other.r, this->i+other.i);
}

// Subtração
ComplexNumber ComplexNumber::sub(ComplexNumber const& other) {
    // Subtrair cada parte
    return ComplexNumber(this->r-other.r, this ->i-other.i);
}

// Multiplicação
ComplexNumber ComplexNumber::prod(ComplexNumber const& other) {
    // Multiplicar as partes reais
    // Multiplicar as partes imaginárias e subtrair (pois i^2 = -1)
    float real = (this->r * other.r) - (this->i * other.i);
    // Multiplicar cada parte imaginária com a real do outro
    float imag = (this->i * other.r) + (this->r * other.i);
    return ComplexNumber(real, imag);
}

// Módulo
float ComplexNumber::abs() {
    // Raiz quadrada da soma dos quadrados das duas partes
    return std::sqrt((this->r*this->r)+(this->i*this->i));
}

// Impressão
std::ostream& operator<< (std::ostream &out, ComplexNumber const& data) {
    // Caso exista parte real
    if (data.r != 0.0) {
        // Imprimir parte real
        out << std::showpos << data.r;
    }

    // Caso exista parte imaginária
    if (data.i != 0.0) {
        // Imprimir sem o sinal explicito
        out << std::showpos << data.i << "i";
    }

    return out;
}
