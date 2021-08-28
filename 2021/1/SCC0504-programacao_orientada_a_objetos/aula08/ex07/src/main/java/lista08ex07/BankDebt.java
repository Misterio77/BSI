package lista08ex07;

/** Um exemplo concreto do Debt
 * Sua função de calcular divida apenas retorna a divida passada, nada mais
 */
public class BankDebt implements Debt {
    private double debt;
    
    public BankDebt(double debt) {
        this.debt = debt;
    }

    public double debt() {
        return this.debt;
    }
}
