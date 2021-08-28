#include <ostream>
#include "devices.hpp"

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


// Construtor do rádio relógio
RadioClock::RadioClock() {
    alarm_station = 0.0;
}
// Alterar alarme do rádio relógio (inclui estação de rádio para despertar)
void RadioClock::set_alarm(Time const& alarm_time, float alarm_station) {
    this->alarm_station = alarm_station;
    Clock::set_alarm(alarm_time);
}
// Imprimir rádio relógio
std::ostream& operator<< (std::ostream &out, RadioClock const& data) {
    // Castar rádio-relógio em relógio e rádio (para chamar suas respectivas funções de impressão)
    const Clock& clock(data);
    const Radio& radio(data);

    // Imprimir rádio, depois relógio
    out << radio << " | " << clock;
    // Caso esteja marcado alarme
    if (clock.get_alarm().has_value()) {
        // Imprimir qual estação de rádio tocará ao despertar
        out << " @ " << data.alarm_station << "Mhz";
    }
    return out;
}
