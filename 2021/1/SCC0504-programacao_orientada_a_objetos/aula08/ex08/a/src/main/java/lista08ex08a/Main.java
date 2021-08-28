package lista08ex08a;

import lista08ex08a.ComputerSystem;
import lista08ex08a.VulnerableComputerSystem;
import lista08ex08a.ProtectedComputerSystem;

public class Main {
    public static void main(String[] args) {
        // Instanciar sistema (que tem uma vulnerabilidade)
        final var vulnerable_system = new VulnerableComputerSystem("senha ultra secreta");

        // A vulnerabilidade existe aqui, então funcionará
        vulnerable_system.unlock("789");
        vulnerable_system.launch_bomb();
        vulnerable_system.lock();

        System.out.println();

        // Instanciar proxy dele, com vulnerabilidade corrigida
        final var protected_system = new ProtectedComputerSystem(vulnerable_system);

        // Isso vai ser bloqueado pelo proxy
        protected_system.unlock("789");
        protected_system.launch_bomb();
        protected_system.lock();
        
    }
}
