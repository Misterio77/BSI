% ==========
% Funções

% Dada uma matriz A, retorna L e D da decomposição LDLT
function [L, D] = ldlt(A)
    len = length(A);
    tmp = zeros(len, 1);
    for i = 1:len
        for j = 1:(i-1)
            tmp(j) = A(i, j)*A(j, j);
        end
        A(i, i) = A(i, i) - A(i, 1:(i-1)) * tmp(1:(i-1));
        A((i+1):len, i) = (A((i+1):len,i) - A((i+1):len, 1:(i-1)) * tmp(1:(i-1)))/A(i, i);
    end

    % Somar parte triangular inferior com matriz identidade
    L = tril(A, -1)+eye(len);
    % Pegar diagonal da A
    D = diag(diag(A));
end

% Resolve x = A\b usando substituição regressiva
function x = regressiva(A, b)
    len = length(A);
    x(len,1) = b(len)/A(len,len);
    for i = (len-1):-1:1
        x(i,1)=(b(i)-A(i,(i+1):len)*x((i+1):len,1))./A(i,i);
    end
end

% Resolve x = A\b usando substituição progressiva
function x = progressiva(A, b)
    len = length(b);
    x(1,1) = b(1)/A(1,1);
    for i = 2:len
        x(i,1)=(b(i)-A(i,1:i-1)*x(1:i-1,1))./A(i,i);
    end
end

% ==========
% Entradas

n = input('');
A = zeros(n);
b = zeros(1, n);
for i = 1:n
    for j = 1:i
        A(i,j) = input('');
        A(j,i) = A(i,j);
    end
end

for i = 1:n
    b(i) = input('');
end

% ==========
% Operações

[L, D] = ldlt(A);
y = progressiva(L,b.');
x = regressiva((D * L.'),y);

% ==========
% Saídas

for i = 1:n
    for j = 1:i
        disp(sprintf('%.6f', L(i, j)));
    end
end
for i = 1:n
    disp(sprintf('%.6f', D(i, i)));
end
for i = 1:n
    disp(sprintf('%.6f', x(i)));
end
