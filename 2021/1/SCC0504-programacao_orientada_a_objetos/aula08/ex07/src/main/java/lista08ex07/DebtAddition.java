package lista08ex07;

import lista08ex07.DebtDecorator;

/** Decorador concreto
 * Esse decorador trata de adicionar uma taxa fixa ao valor da divida
 */
public class DebtAddition extends DebtDecorator {
    private double addition;

    public DebtAddition(Debt inner, double addition) {
        super(inner);
        this.addition = addition;
    }
    public double debt() {
        // Somar adição
        return super.debt()+this.addition;
    }
}
