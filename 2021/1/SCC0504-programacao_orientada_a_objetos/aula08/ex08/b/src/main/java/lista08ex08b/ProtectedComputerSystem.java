package lista08ex08b;

import lista08ex08b.ComputerSystem;
import lista08ex08b.ComputerSystemDecorator;

/**
 * Decorador concreto
 * Esse decorador protege ComputerSystem's da vulnerabilidade 789
 */
public class ProtectedComputerSystem extends ComputerSystemDecorator {
    public ProtectedComputerSystem(ComputerSystem inner) {
        super(inner);
    }
    public void unlock(String password) {
        if (password == "789") {
            password = "987";
        }
        super.unlock(password);
    }
}
