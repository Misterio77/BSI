% Calcula a raiz usando o método de newton
% Recebe como input:
% - Ponto inicial x
% - Função 'f'
% - Erro mínimo 'min_err'
% - Máximo de iterações 'max_its'
function x = newton(x, f, min_err, max_its)
    ft = matlabFunction(diff(f, sym('x')));
    % Iterar até 'max_its' vezes
    for i = 1:max_its
        % Caso a derivada dê 0
        if (ft(x) == 0)
            error("Não é possível prosseguir desse ponto: a derivada é zero")
        end

        x = x - (f(x)/ft(x));

        % Verificar (quase) convergência
        if (abs(f(x)) < min_err)
            return;
        end
    end
    error("Não foi possível convergir")
end
