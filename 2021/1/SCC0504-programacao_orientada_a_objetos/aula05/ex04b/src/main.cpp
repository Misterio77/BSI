#include <iostream>
#include "devices.hpp"
using namespace std;

void polymorphism_example(Device *device) {
    device->turn_on();
}

int main() {
    Clock clock;
    Radio radio;
    polymorphism_example(&clock);
    polymorphism_example(&radio);
    return 0;
}
