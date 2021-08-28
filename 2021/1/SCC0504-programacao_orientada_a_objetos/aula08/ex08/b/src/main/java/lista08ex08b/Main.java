package lista08ex08b;

import lista08ex08b.ComputerSystem;
import lista08ex08b.VulnerableComputerSystem;
import lista08ex08b.ProtectedComputerSystem;

public class Main {
    public static void main(String[] args) {
        // Instanciar sistema (que tem uma vulnerabilidade)
        final var vulnerable_system = new VulnerableComputerSystem("senha ultra secreta");

        // A vulnerabilidade existe aqui, então funcionará
        vulnerable_system.unlock("789");
        vulnerable_system.launch_bomb();
        vulnerable_system.lock();

        System.out.println();

        // Protegê-lo com um decorator
        final var protected_system = new ProtectedComputerSystem(vulnerable_system);

        // A senha será reescrita pelo decorator, bloqueando o acesso
        protected_system.unlock("789");
        protected_system.launch_bomb();
        protected_system.lock();
        
    }
}
