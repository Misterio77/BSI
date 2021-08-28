% IMPORTANTE: Esse arquivo depende da função "build_system" contida em "build_system.m"

% ENTRADA
% Ler conjunto de pontos do arquivo

% Primeiro dataset
% data = importdata('Caso_teste.txt', '\t', 1).data;
% Segundo dataset
data = importdata('Ex07_Points.txt', '\t', 1).data;

% Retirar linhas com NaN (o .txt dado tem uma linha zoada, por ter espaços)
data(any(isnan(data), 2), :) = [];


% Funções que representam cada uma das bases
% Elas tomam pontos x e y como argumentos
bases = {
    @(x,y)(1),
    @(x,y)(x),
    @(x,y)(y),
    @(x,y)(x^2),
    @(x,y)(x*y),
    @(x,y)(y^2),
    @(x,y)(x*y^2),
    @(x,y)(y^3),
};


% Construir sistema
[A, b] = build_system(bases, data);

% Resolver sistema
% Parece que às vezes fica umas poucas casas decimais, então arredondando
coefs = round(A\b * 10^10)/10^10;

% SAÍDA
% Simplesmente imprimir os coficientes
coefs
