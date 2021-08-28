package lista08ex08b;

import lista08ex08b.ComputerSystem;

/** Decorador abstrato
 * Essa classe implementa ComputerSystem
 * e armazena um ComputerSystem dentro de si
 * Suas funções chamarão a função do objeto interno
 */
public abstract class ComputerSystemDecorator implements ComputerSystem {
    private ComputerSystem inner;

    public ComputerSystemDecorator(ComputerSystem inner) {
        this.inner = inner;
    }
    public void unlock(String password) {
        this.inner.unlock(password);
    }
    public void lock() {
        this.inner.lock();
    }
    public void launch_bomb() {
        this.inner.launch_bomb();
    }
}
