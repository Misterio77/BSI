#pragma once

template <typename T>
class Pair {
    private:
        T x;
        T y;
    public:
        // Construtor
        Pair(T x, T y);
        // Maior
        T get_max();
        // Menor
        T get_min();
        // Soma
        T get_sum();
};

template <typename T>
Pair<T>::Pair(T x, T y) {
    this->x = x;
    this->y = y;
}

template <typename T>
T Pair<T>::get_max() {
    return (this->x > this->y)? this->x: this->y;
}

template <typename T>
T Pair<T>::get_min() {
    return (this->x < this->y)? this->x: this->y;
}

template <typename T>
T Pair<T>::get_sum() {
    return (this->x + this->y);
}
