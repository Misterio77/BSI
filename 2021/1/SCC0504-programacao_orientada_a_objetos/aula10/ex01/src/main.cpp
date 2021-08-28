#include <iostream>
#include "pair.hpp"
using namespace std;

int main() {
    Pair<int> pair(3, 5);
    cout << "Maior: " << pair.get_max() << endl;
    cout << "Menor: " << pair.get_min() << endl;
    cout << "Soma: " << pair.get_sum() << endl;
    return 0;
}
