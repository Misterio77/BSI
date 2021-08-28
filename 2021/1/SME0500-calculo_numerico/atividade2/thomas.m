function [y] = thomas(A, f)
    n = length(f);
    v = zeros(n,1);   
    y = v;
    w = A(1,1);
    y(1) = f(1)/w;
    for i=2:n
        v(i-1) = A(i-1,i)/w;
        w = A(i,i) - A(i,i-1)*v(i-1);
        y(i) = ( f(i) - A(i,i-1)*y(i-1) )/w;
    end
    for j=n-1:-1:1
       y(j) = y(j) - v(j)*y(j+1);
    end
endfunction
