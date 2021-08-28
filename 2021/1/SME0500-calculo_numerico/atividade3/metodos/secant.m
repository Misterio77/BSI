% Calcula a raiz usando o método da secante
% Recebe como input:
% - Pontos iniciais 'x0' e 'x0'
% - Função 'f'
% - Erro mínimo 'min_err'
% - Máximo de iterações 'max_its'
% - Modo benchmark (imprime valores a cada iteração, e iterações totais)
function x1 = secant(x0, x1, f, min_err, max_its, benchmark = false)
    for i = 1:max_its
        % Verificar que a secante não é horizontal
        % Caso seja, acontecerá divisão por 0
        if (f(x0) == f(x1))
            error("A secante não pode ter inclinação 0")
        end

        % Calcular novo termo
        new = x1 - f(x1) * ( (x1 - x0)/(f(x1) - f(x0)) );
        % O x0 será o antigo x1
        x0 = x1;
        % x1 será o novo termo
        x1 = new;
        if benchmark
            disp(sprintf("%d\t %d\t %d", x0, x1, f(x1)));
        end 

        % Verificar (quase) convergencia
        if (abs(f(x1)) < min_err)
            if benchmark
                disp(["Iterações: ", int2str(i)]);
            end 
            return;
        end
    end
    error("Não foi possível convergir")
end
