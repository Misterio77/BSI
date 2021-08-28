package lista08ex07;

import lista08ex07.Debt;

/** Decorador abstrato
 * Essa classe implementa Debt, e armazena um Debt dentro de si
 * Sua função de calcular debito chamará a função de calcular
 * do Debt interno
 */
public abstract class DebtDecorator implements Debt {
    private Debt inner;

    public DebtDecorator(Debt inner) {
        this.inner = inner;
    }
    public double debt() {
        return inner.debt();
    }
}
