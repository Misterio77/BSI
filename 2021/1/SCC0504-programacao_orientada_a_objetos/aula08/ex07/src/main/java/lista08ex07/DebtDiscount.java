package lista08ex07;

import lista08ex07.DebtDecorator;

/** Decorador concreto
 * Esse decorador trata de fazer um desconto percentual no
 * montante do empréstimo.
 */
public class DebtDiscount extends DebtDecorator {
    private double discount_pct;

    public DebtDiscount(Debt inner, double discount_pct) {
        super(inner);
        this.discount_pct = discount_pct;
    }
    public double debt() {
        // Desconto em decimal
        final var discount = this.discount_pct / 100.0;
        return super.debt()*(1-discount);
    }
}
