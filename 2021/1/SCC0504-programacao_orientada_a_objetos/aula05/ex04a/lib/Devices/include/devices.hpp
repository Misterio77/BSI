#pragma once
#include <ostream>
#include <optional>

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
        friend std::ostream& operator<< (std::ostream &out, Time const& data);
};

// Representa um relógio
class Clock {
    private:
        // Horário atual
        Time current_time;
        // Horário do alarme
        std::optional<Time> alarm_time;
    public:
        // Construtor
        Clock();
        // Obter horário
        Time get_time() const;
        // Alterar horário
        void set_time(Time const& current_time);
        // Obter alarme, caso setado
        std::optional<Time> get_alarm() const;
        // Alterar alarme
        void set_alarm(Time const& alarm_time);
        // Desabilitar o alarme;
        void disable_alarm();
        // Impressão
        friend std::ostream& operator<< (std::ostream &out, Clock const& data);
};

// Representa um rádio
class Radio {
    private:
        // Frequência da estação atual
        float frequency;
        // True para FM, false para AM
        bool type;
    public:
        // Construtor
        Radio();
        // Alterar frequencia
        void set_frequency(float frequency);
        // Alterar para AM
        void set_am();
        // Alterar para FM
        void set_fm();
        // Impressão
        friend std::ostream& operator<< (std::ostream &out, Radio const& data);
};

// Representa um rádio relógio
class RadioClock: public Radio, public Clock {
    private:
        // Estação que será usada no alarme
        float alarm_station;
    public:
        // Construtor
        RadioClock();
        // Alterar horário do alarme, especificando qual frequencia despertar
        void set_alarm(Time const& alarm_time, float alarm_station);
        // Impressão
        friend std::ostream& operator<< (std::ostream &out, RadioClock const& data);
};
