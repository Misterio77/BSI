package lista08ex09;

/**
 * Classe abstrata, contém alguns comportamentos comuns dos calculators
 */
public abstract class CalculatorCommon implements Calculator {
    // Armazena o proximo na chain
    private Calculator nextCalculator;

    // Altera o proximo
    public void setNext(Calculator nextCalculator) {
        this.nextCalculator = nextCalculator;
    }
    // Chama sqrt no proximo
    public double sqrt(int number) {
        if (this.nextCalculator != null) {
            return this.nextCalculator.sqrt(number);
        } else {
            return 1.0;
        }
    }
    // Chama square no proximo
    public double square(int number) {
        if (this.nextCalculator != null) {
            return this.nextCalculator.square(number);
        } else {
            return 1.0;
        }
    }
}
