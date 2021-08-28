#pragma once
#include <ostream>
#include <optional>
using namespace std;

class Device {
    private:
        string manufacturer;
    public:
        // Construtor vazio
        Device();
        // Obter fabricante
        string get_manufacturer() const;
        // Alterar fabricante
        void set_manufacturer(string manufacturer);
        // Ligar
        virtual void turn_on() = 0;
        // Desligar
        virtual void turn_off() = 0;
};

// Representa um horário
class Time {
    private:
        // Hora
        int h;
        // Minuto
        int m;
        // Segundo
        int s;
    public:
        // Construtor com números dados
        Time(int hour, int minute, int second);
        // Construtor vazio (todos números = 0)
        Time();
        // Impressão
        friend ostream& operator<< (ostream &out, Time const& data);
};

// Representa um relógio
class Clock: public Device {
    private:
        // Horário atual
        Time current_time;
        // Horário do alarme
        optional<Time> alarm_time;
    public:
        // Ligar
        void turn_on();
        // Desligar
        void turn_off();
        // Construtor
        Clock();
        // Obter horário
        Time get_time() const;
        // Alterar horário
        void set_time(Time const& current_time);
        // Obter alarme, caso setado
        optional<Time> get_alarm() const;
        // Alterar alarme
        void set_alarm(Time const& alarm_time);
        // Desabilitar o alarme;
        void disable_alarm();
        // Impressão
        friend ostream& operator<< (ostream &out, Clock const& data);
};

// Representa um rádio
class Radio: public Device {
    private:
        // Frequência da estação atual
        float frequency;
        // True para FM, false para AM
        bool type;
    public:
        // Ligar
        void turn_on();
        // Desligar
        void turn_off();
        // Construtor
        Radio();
        // Alterar frequencia
        void set_frequency(float frequency);
        // Alterar para AM
        void set_am();
        // Alterar para FM
        void set_fm();
        // Impressão
        friend ostream& operator<< (ostream &out, Radio const& data);
};
