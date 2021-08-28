package lista08ex10;

import lista08ex10.Observer;

public class UppercaseObserver implements Observer {
    public String run(String input) {
        return input.toUpperCase();
    }
}
