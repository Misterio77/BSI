% IMPORTANTE: Esse arquivo depende do script "solve.m", que por sua vez depende da função "build_system.m"

% Chamar o script da 'solve.m', ela guarda a solução em 'coefs'
solve;

% Calcuma uma matriz de pontos Z para vetores de pontos X e Y
function B = get_z(X, Y, coefs, bases)
    x_len = length(X);
    y_len = length(Y);

    % Para cada x
    for i = 1:x_len
        % Para cada y
        for j = 1:y_len
            % Calcular valor e colocar na matriz
            B(j,i) = get_single_z(X(i), Y(j), coefs, bases);
        end
    end
end

% Calcula o valor z do polinomio, dado seu vetor de coeficientes,
% conjunto de bases, e valor de um x e um y
function z = get_single_z(x, y, coefs, bases)
    bases_len = length(bases);
    z = 0;
    for i = 1:bases_len
        z += coefs(i) * bases{i}(x, y);
    end
end

% Criar malha de pontos X e Y
% Malha ideal para o primeiro dataset
% X = (0:0.05:0.95);
% Y = (0:0.05:1.9);

% Malha ideal para o segundo dataset
X = (0:0.05:2.33);
Y = (0:0.05:2);

% Coms seus Zs adquiridos do polinomio ajustado
Z = get_z(X, Y, A\b, bases);

% Criar figura
figure
% Gráfico da sueperfície, dado pelos pontos de X, Y e Z
surf(X, Y, Z)
hold on
% Adicionar os pontos do dataset
plot3(data(:,1), data(:,2), data(:,3), 'o', 'MarkerFaceColor', 'r')
