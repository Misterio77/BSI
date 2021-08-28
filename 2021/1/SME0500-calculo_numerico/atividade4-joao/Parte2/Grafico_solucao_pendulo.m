##  Os códigos foram todos postos juntos porque não consegui entender como usar 
##  funções locais no octave, e o run.codes só permite a submissão de um arquivo .m
##  de forma que não posso usar funções em outros arquivos (newton_sis.m e 
##  thomas_algorithm.m, por exemplo). Por isso, o código está "feio" rsrs
##  Aluno:  João Vitor Diógenes     NumUSP: 11816122

  % Entrada:
  % T - Periodo final
  % m - numero de pontos interiores, correspondendo tambem a
  %     dimensao da matriz J do sistema linear J*delta=-G
  % alpha - posicao inicial do pendulo
  % beta - posicao final do pendulo
  % k - numero de iteracoes
  % theta_0 - chute inicial

  T = input('');
  m = input('');
  alpha = input('');
  beta = input('');
  k = input('');

  for i = 1:m
      theta_0(i) = input('');
  end

  ##  ALGORITMO
  % Construa os vetores theta e G, e a matriz J.
  % Tendo em maos J e G, voce deve adicionar um codigo que resolva o
  % sistema J*delta = -G atraves do algoritmo de Thomas, lembrando de
  % atualizar o valor de theta atraves de delta.

  diagonal = zeros(m);
  diagonalInferior = zeros(m - 1);

  h = T / (m + 1);
  ## preenchendo a diagonal a partir da equacao da jacobeana para j == i
  for i = 1 : m
    diagonal(i) = (-2 / (h * h) ) + cos(theta_0(i));
  endfor
  ## preenchendo a diagonal inferior da jacobeana
  for i = 1 : m - 1
    diagonalInferior(i) = 1 / (h * h);
  endfor

  G = zeros(m);

  ## Calculando G
  G(1) = ( (alpha - 2 * theta_0(1) + theta_0(2) ) / (h * h) ) + sin(theta_0(1));
  for i = 2 : m - 1
    G(i) = ( (theta_0(i - 1) - 2 * theta_0(i) + theta_0(i + 1)) / (h * h) ) + sin(theta_0(i));
  endfor
  G(m) = ( ( theta_0(m - 1) - 2 * theta_0(m) + beta) / (h * h) ) + sin(theta_0(m));
  ## Multiplicando G por -1 para ser o lado direito do nosso sistema J * delta = -G
  G = -1 * G;
  
  delta = zeros(m);
  
  ## Método de Newton para solucionar o sistema
  for counter = 1 : k
    ## Thomas algorithm 
          n = length(G);
          % Entrada:
          % n  - dimensao da matriz A e do sistema linear Ax=b

          % d - entradas da diagonal principal da matriz SPD tridiagonal
          % below - entradas da diagonal inferior (e superior ja que e simetrica)
          % B  - vetor lado-direito do sistema Ax=b

          below(1) = 0;
          for i = 2:n
              below(i,1) = diagonalInferior(i-1);
          end

          for i = 1: n - 1
              above(i) = below(i + 1);
          end

          above(1) = above(1) / diagonal(1);
          G(1) = G(1) / diagonal(1);

          for i = 2: n - 1
              above(i) = above(i) / ( diagonal(i) - ( below(i) * above(i - 1) ) );
              G(i) = ( G(i) - ( below(i) * G(i - 1) ) ) / ( diagonal(i) - ( below(i) * above(i - 1) ) );
          end

          G(n) = ( G(n) - ( below(n) * G(n - 1) ) )/ (diagonal(n) - (below(n) * above(n - 1) ) );

          delta(n) = G(n);

          for i = n - 1: -1: 1
              delta(i) = G(i) - ( above(i) * delta(i + 1) );
          end   
    ## fim do thomas algorithm
    
    ## Calculando os valores a partir do delta obtido, ou seja, "resultado de uma iteracao"
    for j = 1 : m
      theta_0(j) = theta_0(j) + delta(j);
    endfor
    
    ## preenchendo a diagonal a partir da equacao da jacobeana para j == i
    for i = 1 : m
      diagonal(i) = (-2 / (h * h) ) + cos(theta_0(i));
    endfor
    ## Nao precisamos atualizar os valores da diagonal inferior, porque são
    ## "constantes" (nao dependem do valor de theta_0)
    
    ## Recalculando G para a proxima iteracao
    G(1) = ( (alpha - 2 * theta_0(1) + theta_0(2) ) / (h * h) ) + sin(theta_0(1));
    for i = 2 : m - 1
      G(i) = ( (theta_0(i - 1) - 2 * theta_0(i) + theta_0(i + 1)) / (h * h) ) + sin(theta_0(i));
    endfor
    G(m) = ( ( theta_0(m - 1) - 2 * theta_0(m) + beta) / (h * h) ) + sin(theta_0(m));
    G = -1 * G;
##    printf("Residuo na %d iteracao: %.16f\n", counter, norm(delta));
    printf("Na iteracao %d, o theta vale: ", counter);
    theta_0
  endfor
    
  ##  SAÍDAS
  ## preenchendo o theta final com os valores de alpha e beta
  theta = zeros(m + 2);
  theta(1) = alpha;
  theta(m + 2) = beta;
  for i = 2 : m + 1
    theta(i) = theta_0(i - 1);
  endfor
  
  % Apos o algoritmo, voce deve imprimir o vetor solucao theta e o erro
  % da iteração k da seguinte forma:
  for i = 1 : m + 2
    printf('%.16f\n', theta(i));
  end
  printf('%.16f\n', norm(delta));