package lista08ex09;

import lista08ex09.Calculator;
import lista08ex09.CalculatorCommon;

import java.lang.Math;
/**
 * Handler que cuida de qualquer número
 */
public class CalculatorOthers extends CalculatorCommon implements Calculator {
    public double sqrt(int number) {
        // Calcular usando sqrt do java
        var result = Math.sqrt(number);

        // Arredondar valor para 5 casas decimais
        result = Math.round(result * 100000.0) / 100000.0;

        // Tratamos "tudo", então o numero ficou 1
        // Retornamos nosso result multiplicado pelo o que o pai quiser fazer
        return super.sqrt(1) * result;
    }
    public double square(int number) {
        // Calcular da forma usual
        final var result = number*number;

        // Tratamos "tudo", então o numero ficou 1
        // Retornamos nosso result multiplicado pelo o que o pai quiser fazer
        return super.sqrt(1) * result;
    }
}
