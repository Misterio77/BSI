function [x,k] = newton(func, dfunc, x, tol, kmax)
% dfunc: derivada da funcao func(x)
% x: chute inicial x0

  for k = 1 : kmax
    dx = func(x) / dfunc(x);
    x = x - dx;
    
    if (abs(dx) < tol)
      return;
    end
  end
  
##  x = NaN;

endfunction
