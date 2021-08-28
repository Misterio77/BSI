% ==========
% Entradas:

% Periodo final
T = input('');
% numero de pontos interiores, correspondendo tambem a
% dimensao da matriz J do sistema linear J*delta=-G
m = input('');
% posicao inicial do pendulo
alpha = input('');
% posicao final do pendulo
beta = input('');
% numero de iteracoes
k = input('');
% chute inicial
for i = 1:m
    theta_0(i) = input('');
end

% ==========
% Funções

% Algoritmo de Thomas
function x = thomas(A, f)
    len = length(f);

    v = zeros(len,1);   

    w = A(1,1);

    x = v;
    x(1) = f(1)/w;
    
    for i=2:len
        v(i-1) = A(i-1,i)/w;
        w = A(i,i) - A(i,i-1)*v(i-1);
        x(i) = ( f(i) - A(i,i-1)*x(i-1) )/w;
    end

    for j=len-1:-1:1
       x(j) = x(j) - v(j)*x(j+1);
    end
end

% ==========
% Operações:

h = T/(m+1);

% Colocar diagonal superior e inferior de J
for i = 1:m-1
    J(i+1, i) = 1/(h*h);
    J(i, i+1) = 1/(h*h);
end

% Iterações
for i = 1:k
    % (re)calcular diagonal principal de J
    for j = 1:m
        J(j,j) = -2/(h*h) + cos(theta_0(j));
    end

    % (re)calcular G
    G(1) = (alpha - 2*theta_0(1) + theta_0(2))/(h*h) + sin(theta_0(1));
    for j = 2:m-1
        G(j) = (theta_0(j-1) - 2*theta_0(j) + theta_0(j+1))/(h*h) + sin(theta_0(j));
    end
    G(m) = (theta_0(m-1) - 2*theta_0(m) + beta)/(h*h) + sin(theta_0(m));

    % Resolver J*delta = -G
    delta = thomas(J, -1 * G);

    % Somar valores de delta em theta
    theta_0 += delta';
end

% Preencher theta final
theta = [ alpha theta_0 beta ];

% ==========
% Saídas:
for i = 1:m+2
  printf('%.6f\n', theta(i));
end
printf('%.6f\n', norm(delta));
