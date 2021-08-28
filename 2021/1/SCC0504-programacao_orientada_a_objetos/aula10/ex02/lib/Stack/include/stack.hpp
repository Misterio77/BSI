#pragma once
#include <deque>

template <typename T>
class Stack {
    private:
        std::deque<T> inner;
        unsigned int len;
    public:
        Stack();
        void push(T input);
        T pop();
};

template <typename T>
Stack<T>::Stack() {
    std::deque<T> inner;
    len = 0;
}

template <typename T>
void Stack<T>::push(T input) {
    inner.push_front(input);
    len++;
}
template <typename T>
T Stack<T>::pop() {
    if (len > 0) {
        T value = inner.front();
        inner.pop_front();
        len--;
        return value;
    } else {
        throw std::invalid_argument("Essa pilha não tem elementos");
    }
}
