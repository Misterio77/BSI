package lista08ex10;

import lista08ex10.Observer;

import java.time.Year;

public class YearObserver implements Observer {
    public String run(String input) {
        return (input + Year.now().getValue());
    }
}
