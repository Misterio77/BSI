#include <ostream>
#include <iostream>
#include "devices.hpp"

Device::Device() {
    manufacturer = "";
}
string Device::get_manufacturer() const {
    return this->manufacturer;
}
void Device::set_manufacturer(string manufacturer) {
    this->manufacturer = manufacturer;
}

// Construtor de um horário
Time::Time(int hour, int minute, int second) {
    h = hour;
    m = minute;
    s = second;
}
// Construtor sem argumentos
Time::Time() {
    h = 0;
    m = 0;
    s = 0;
}
// Imprimir horário
std::ostream& operator<< (std::ostream &out, Time const& data) {
    out << data.h << ":" << data.m << ":" << data.s;
    return out;
}


// Construtor relógio
Clock::Clock() {
    alarm_time = std::nullopt;
}
void Clock::turn_off() {
    cout << "Desligando relógio..." << endl;
}
void Clock::turn_on() {
    cout << "Ligando relógio..." << endl;
}
// Obter horário atual do relógio
Time Clock::get_time() const {
    return this->current_time;
}
// Alterar horário do relógio
void Clock::set_time(Time const& current_time) {
    this->current_time = current_time;
}
// Obter horário do alarme do relógio
std::optional<Time> Clock::get_alarm() const {
    return this->alarm_time;
}
// Alterar horário do alarme do relógio
void Clock::set_alarm(Time const& alarm_time) {
    this->alarm_time = alarm_time;
}
// Desligar alarme
void Clock::disable_alarm() {
    this->alarm_time = std::nullopt;
}
// Imprimir relógio
std::ostream& operator<< (std::ostream &out, Clock const& data) {
    out << "Horário atual: " << data.current_time;
    std::optional<Time> alarm = data.get_alarm();
    if (alarm.has_value()) {
        out << " | " << "Alarme: " << *alarm;
    }
    return out;
}


// Construtor rádio
Radio::Radio() {
    frequency = 0.0;
    type = false;
}
void Radio::turn_off() {
    cout << "Desligando rádio..." << endl;
}
void Radio::turn_on() {
    cout << "Ligando rádio..." << endl;
}
// Alterar frequência do rádio
void Radio::set_frequency(float frequency) {
    this->frequency = frequency;
}
// Trocar rádio para AM
void Radio::set_am() {
    this->type = false;
}
// Trocar rádio para FM
void Radio::set_fm() {
    this->type = true;
}
// Imprimir rádio
std::ostream& operator<< (std::ostream &out, Radio const& data) {
    out << "Estação atual: " << data.frequency << "MHz";
    if (data.type) {
        out << " (FM)";
    } else {
        out << " (AM)";
    }
    return out;
}
