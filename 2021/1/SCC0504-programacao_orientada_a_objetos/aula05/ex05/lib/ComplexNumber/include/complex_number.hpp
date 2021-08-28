#pragma once
#include <ostream>
#include <cmath>

// Declaração da classe do numero complexo (implementação está na pasta src)
class ComplexNumber {
    private:
        float r;
        float i;
    public:
        // Construtor
        ComplexNumber(float r, float i);
        // Soma
        ComplexNumber operator+(ComplexNumber const& other);
        // Subtração
        ComplexNumber operator-(ComplexNumber const& other);
        // Multiplicação
        ComplexNumber operator*(ComplexNumber const& other);
        // Módulo
        friend float abs(ComplexNumber const& number);
        // Impressão
        friend std::ostream& operator<< (std::ostream &out, ComplexNumber const& data);
};

