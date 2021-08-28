% Dadas bases (cell array de funções) e dados (matriz, onde cada linha é um ponto),
% constrói o sistema (utilizando produto escalar)
function [A, b] = build_system(bases, data)
    bases_len = length(bases);
    data_len = size(data, 1);

    % Computar bases, usando as funções que representam cada uma
    for i = 1:bases_len
        % Para cada ponto do conjunto de dados
        for j = 1:data_len
            % Pegar x e y (posição 1 e 2)
            x = data(j,1);
            y = data(j,2);

            % Construir uma matriz de bases computadas
            % Cada linha é um ponto do conj de dados
            % Cada coluna é uma base
            computed_bases(j, i) = bases{i}(x,y);
        end
    end

    % Para cada base
    for i = 1:bases_len
        % Preencher posição de b
        % <base_i, z>
        b(i,1) = dot(computed_bases(:,i), data(:,3));
        % Preencher uma linha em A
        for j = 1:bases_len
            % <base_i, base_j>
            A(i,j) = dot(computed_bases(:,i), computed_bases(:,j));
        end
    end
end
