package lista08ex09;

import lista08ex09.Calculator;
import lista08ex09.CalculatorCommon;

/**
 * Handler que cuida de números multiplos de 2
 */
public class CalculatorTwo extends CalculatorCommon implements Calculator {
    public double sqrt(int number) {
        var result = 1.0;
        // Caso seja divisivel por dois, podemos tratar
        while (number % 2.0 == 0) {
            number /= 2;
            result = 1.41421356237 * result;
        }

        // Arredondar valor para 5 casas decimais
        result = Math.round(result * 100000.0) / 100000.0;

        // Pedir para o pai chamar o proximo da corrente no nosso numero
        // Retornando o nosso result multiplicado por o que der lá
        return super.sqrt(number) * result;
    }
    public double square(int number) {
        var result = 1.0;
        // Caso seja divisivel por dois, podemos tratar
        while (number % 2.0 == 0) {
            number /= 2;
            result = result * 4;
        }

        // Pedir para o pai chamar o proximo da corrente no nosso numero
        // Retornando o nosso result multiplicado por o que der lá
        return super.square(number) * result;
    }
}
