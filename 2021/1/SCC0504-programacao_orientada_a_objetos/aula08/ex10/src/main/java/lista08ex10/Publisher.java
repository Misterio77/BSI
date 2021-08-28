package lista08ex10;

import lista08ex10.Observer;

import java.util.ArrayList;

public class Publisher {
    private ArrayList<Observer> observers;

    public Publisher() {
        this.observers = new ArrayList<>();
    }

    /**
    * Inscrever um observador ao publisher
    */
    public boolean subscribe(Observer observer) {
        // Caso já esteja inscrito
        if (this.observers.size() != 0 && this.observers.contains(observer)) {
            return false;
        }

        this.observers.add(observer);
        return true;
    }
    /**
    * Desinscrever um observador do publisher
    */
    public boolean unsubscribe(Observer observer) {
        final var index = this.observers.indexOf(observer);
        // Caso não esteja inscrito
        if (index == -1) {
            return false;
        }

        this.observers.remove(index);
        return true;
    }
    /**
    * Notificar todos os observadores
    */
    public ArrayList<String> notify(String input) {
        // Caso não tenhamos nenhum inscrito
        if (this.observers.size() == 0) {
            return null;
        }

        var outputs = new ArrayList<String>();
        // Para cada observador
        for (Observer observer: this.observers) {
            final var output = observer.run(input);
            outputs.add(output);
        }
        return outputs;
    }
}
