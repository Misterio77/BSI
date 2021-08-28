#include <iostream>
#include <cstdlib>
#include <vector>

#include "complex_number.hpp"
using namespace std;

int main() {
    // Criar vetor (na heap)
    vector<ComplexNumber> *numbers = new vector<ComplexNumber>;

    // Semear o gerador aleatório
    srand(time(0));

    // Gerar 100 números
    for (int i = 0; i < 100; ++i) {
        // Gerar número de -100 a 100
        float real = (rand() % 200) - 100;
        float imag = (rand() % 200) - 100;
        numbers->push_back(ComplexNumber(real, imag));
    }

    // Criar um número complexo, inicialmente 0
    ComplexNumber sum(0, 0);
    for (auto it = numbers->begin(); it != numbers->end(); ++it) {
        // Somar
        sum = sum + *it;
    }

    // Desalocar vetor
    delete numbers;

    // Imprimir soma
    cout << "Soma: " << sum << endl;
    return 0;
}
