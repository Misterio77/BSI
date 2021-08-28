% Calcula a raíz usando o método da falsa posição
% Recebe como input:
% - Limites 'a' e 'b' do intervalo
% - Função 'f'
% - Erro mínimo 'min_err'
% - Máximo de iterações 'max_its'
% - Modo benchmark (imprime valores a cada iteração, e iterações totais)
function c = regula_falsi(a, b, f, min_err, max_its, benchmark = false)
    % Verificar que a reta é cruzada
    if (sign(f(a)) == sign(f(b)))
        error("Os dois extremos precisam cruzar a reta x")
    end

    % Iterar até 'max_its' vezes
    for i = 1:max_its
        % Essa equação tem origem ao igualar os
        % coeficiente das retas b-c e a-c,
        c = b - (f(b) * ((b-a)/(f(b)-f(a))));

        if benchmark
            disp(sprintf("%d\t %d\t %d\t %d", a, b, c, f(c)));
        end 
        % Escolher onde substituir com o c baseado
        % nos sinais (a e b devem ter sinais opostos)
        if sign(f(c)) == sign(f(a))
            a = c;
        else
            b = c;
        end

        % Verificar (quase) convergência
        if (abs(f(c)) < min_err)
            if benchmark
                disp(["Iterações: ", int2str(i)]);
            end 
            return;
        end
    end
    error("Não foi possível convergir")
end
