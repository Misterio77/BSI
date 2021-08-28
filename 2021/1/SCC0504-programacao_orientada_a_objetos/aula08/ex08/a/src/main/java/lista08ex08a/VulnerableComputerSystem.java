package lista08ex08a;

import lista08ex08a.ComputerSystem;

/**
 * Sistema de computador, que tem uma vulnerável (senha ser 789)
 */
public class VulnerableComputerSystem implements ComputerSystem{
    private String system_password;
    private boolean system_unlocked;

    public VulnerableComputerSystem(String system_password) {
        this.system_unlocked = false;
        this.system_password = system_password;
    }

    public void unlock(String password) {
        if (password == system_password) {
            System.out.println("Sistema desbloqueado com a senha correta");
            this.system_unlocked = true;
        } else if  (password == "789") {
            System.out.println("Vulnerabilidade, sistema desbloqueado");
            this.system_unlocked = true;
        } else {
            System.out.println("ERRO: Senha incorreta");
        }
    }

    public void lock() {
        this.system_unlocked = false;
    }

    public void launch_bomb() {
        if (this.system_unlocked) {
            System.out.println("Lançando bomba atômica...");
        } else {
            System.out.println("ERRO: O sistema está bloqueado");
        }
    }
}
