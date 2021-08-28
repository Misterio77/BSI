% Calcula a raiz de uma função pelo metodo do ponto fixo
% Recebe como input:
% - Ponto inicial x
% - Função 'f'
% - Função 'g'
% - Erro mínimo 'min_err'
% - Máximo de iterações 'max_its'
% - Modo benchmark (imprime valores a cada iteração, e iterações totais)
function x = fixed_point(x, f, g, min_err, max_its, benchmark = false)
    % Iterar até 'max_its' vezes
    for i = 1:max_its
        % Calcular próximo elemento
        x = g(x);
        if benchmark
            disp(sprintf("%d\t %d\t %d", x, g(x), f(x)));
        end 

        % Verificar (quase) convergência
        if (abs(f(x)) < min_err)
            if benchmark
                disp(["Iterações: ", int2str(i)]);
            end 
            return;
        end
    end
    error("Não foi possível convergir")
end
