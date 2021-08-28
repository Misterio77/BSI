package lista08ex08a;

import lista08ex08a.ComputerSystem;
import lista08ex08a.VulnerableComputerSystem;

/**
 * Classe proxy, implementa os mesmos métodos de ComputerSystem
 * e chama os métodos equivalentes de VulnerableComputerSystem, tirando a
 * vulnerabilidade.
 */
public class ProtectedComputerSystem implements ComputerSystem {
    private VulnerableComputerSystem inner_system;

    public ProtectedComputerSystem(VulnerableComputerSystem inner) {
        this.inner_system = inner;
    }

    public void unlock(String password) {
        if (password == "789") {
            System.out.println("Vulnerabilidade bloqueada.");
            return;
        }
        this.inner_system.unlock(password);
    }
    public void lock() {
        this.inner_system.lock();
    }
    public void launch_bomb() {
        this.inner_system.launch_bomb();
    }
}
