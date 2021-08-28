package lista08ex07;

import lista08ex07.DebtDecorator;

import java.lang.Math;

/** Decorador concreto
 * Esse decorador trata de adicionar juros compostos mensais à
 * dívida.
 */
public class DebtInterest extends DebtDecorator {
    // Juros em porcentagem
    private double interest_pct;
    // Número de meses
    private int months;

    public DebtInterest(Debt inner, double interest_pct, int months) {
        super(inner);
        this.interest_pct = interest_pct;
        this.months = months;
    }
    public double debt() {
        // Juros em decimal
        final var interest = this.interest_pct / 100.0;
        // Fórmula do juro composto A = P*(1+r)^t
        return super.debt()*Math.pow((1+interest), months);
    }
}
