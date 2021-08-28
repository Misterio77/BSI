package lista08ex09;

/**
 * Interface que representa todos os Handlers
 * Vulgo, classes que podem fazer a conta
 */
public interface Calculator {
    /**
     * Esse método permite marcar qual o próximo handle a ser usado na chain
     */
    public void setNext(Calculator handle);
    public double sqrt(int number);
    public double square(int number);
}
