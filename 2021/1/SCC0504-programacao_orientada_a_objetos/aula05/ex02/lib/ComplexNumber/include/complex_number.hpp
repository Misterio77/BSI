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
        ComplexNumber sum(ComplexNumber const& other);
        // Subtração
        ComplexNumber sub(ComplexNumber const& other);
        // Multiplicação
        ComplexNumber prod(ComplexNumber const& other);
        // Módulo
        float abs();
        // Impressão
        friend std::ostream& operator<< (std::ostream &out, ComplexNumber const& data);
};

