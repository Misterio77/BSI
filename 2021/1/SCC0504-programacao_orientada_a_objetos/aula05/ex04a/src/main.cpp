#include <iostream>
#include "devices.hpp"
using namespace std;

int main() {
    // Instanciar
    RadioClock radio_relogio;

    // Ajustar horário
    radio_relogio.set_time(Time(21, 59, 0));

    // Trocar pra FM
    radio_relogio.set_fm();
    // Sintonizar uma rádio
    radio_relogio.set_frequency(96.1);

    // Colocar um alarme
    radio_relogio.set_alarm(Time(6, 0, 0), 95.3);


    cout << radio_relogio << endl;
    return 0;
}
