package lista08ex09;

import lista08ex09.CalculatorTwo;
import lista08ex09.CalculatorThree;
import lista08ex09.CalculatorOthers;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // Primeiro handler, cuida de multiplos de 2
        final var h1 = new CalculatorTwo();
        // Primeiro handler, cuida de multiplos de 3
        final var h2 = new CalculatorThree();
        // Primeiro handler, cuida de qualquer numero
        final var h3 = new CalculatorOthers();

        h1.setNext(h2);
        h2.setNext(h3);

        System.out.print("Digite um número: ");
        Scanner in = new Scanner(System.in);
        final var num = in.nextInt();
        in.close();

        System.out.println("Sua raiz: "+ h1.sqrt(num));
        System.out.println("Seu quadrado: "+ h1.square(num));
    }
}
