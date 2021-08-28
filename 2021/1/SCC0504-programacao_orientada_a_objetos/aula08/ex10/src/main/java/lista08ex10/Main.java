package lista08ex10;

import lista08ex10.Observer;
import lista08ex10.YearObserver;
import lista08ex10.SizeObserver;
import lista08ex10.UppercaseObserver;
import lista08ex10.Publisher;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // Instanciar publisher
        final var publisher = new Publisher();

        // Instanciar e inscrever observadores
        publisher.subscribe(new UppercaseObserver());
        publisher.subscribe(new YearObserver());
        publisher.subscribe(new SizeObserver());

        // Ler texto do usuário
        Scanner in = new Scanner(System.in);
        System.out.print("Digite seu texto: ");
        final var text = in.nextLine();
        in.close();

        // Notificar observadores e imprimir os inputs deles
        final var outputs = publisher.notify(text);
        System.out.println(outputs);
    }
}
