package lista08ex07;

import lista08ex07.BankDebt;
import lista08ex07.DebtInterest;
import lista08ex07.DebtDiscount;
import lista08ex07.DebtAddition;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);

        // Ler valor base
        System.out.print("Digite o valor da dívida: ");
        final var debt_amount = in.nextInt();

        // Ler juros
        System.out.println();
        System.out.print("Digite a % do juros mensal: ");
        final var interest = in.nextInt();
        System.out.print("Digite o número de meses da dívida: ");
        final var months = in.nextInt();

        // Ler desconto
        System.out.println();
        System.out.print("Digite a % do desconto: ");
        final var discount = in.nextInt();

        // Ler acréscimo
        System.out.println();
        System.out.print("Digite o valor do acréscimo: ");
        final var addition = in.nextInt();

        in.close();
        System.out.println("=====");

        // Criar dívida com BankDebt
        // que implementa Debt
        final var d0 = new BankDebt(debt_amount);
        System.out.println("Dívida base: "+d0.debt());

        // Decorar BankDebt com DebtInterest
        // que é um decorator concreto cujo pai (DebtDecorator) implementa Debt
        final var d1 = new DebtInterest(d0, interest, months);
        System.out.println("Dívida com juros: "+d1.debt());

        // Decorar novamente, agora com DebtDiscount
        // que também é um decorator com DebtDecorator como pai
        final var d2 = new DebtDiscount(d1, discount);
        System.out.println("Dívida com juros e desconto: "+d2.debt());

        // Decorar novamente, agora com DebtAddition
        // que também é um decorator com DebtDecorator como pai
        final var d3 = new DebtAddition(d2, addition);
        System.out.println("Dívida com juros, desconto, e acréscimo: "+d3.debt());
    }
}
