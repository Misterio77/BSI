#include <iostream>
#include "complex_number.hpp"
using namespace std;

int main() {
    ComplexNumber number1(1.0, 2.0);
    ComplexNumber number2(-1.0, 3.0);

    cout << "n1 = " << number1 << endl;
    cout << "n2 = " << number2 << endl;
    cout << "n1+n2 = " << number1.sum(number2) << endl;
    cout << "n1-n2 = " << number1.sub(number2) << endl;
    cout << "n1*n2 = " << number1.prod(number2) << endl;
    cout << "|n1| = " << number1.abs() << endl;
    return 0;
}
