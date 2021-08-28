n = input('');
for i = 1:n
    d0(i,1) = input('');
end

for i = 1:n-1
    d1(i+1,1) = input('');
    d2(i,1) = d1(i+1,1);
end

for i = 1:n
    b(i,1) = input('');
end

function [x] = algoritmo_thomas(a, b, c, d)
    n = length(d);
    v = zeros(n,1);   
    x = v;
    w = a(1);
    x(1) = d(1)/w;
    for i=2:n
        v(i-1) = c(i-1)/w;
        w = a(i) - b(i)*v(i-1);
        x(i) = ( d(i) - b(i)*x(i-1) )/w;
    end
    for j=n-1:-1:1
       x(j) = x(j) - v(j)*x(j+1);
    end
endfunction

x = algoritmo_thomas(d0, d1, d2, b);

for i = 1:n
    format('long', 'e');
    disp(x(i));
end
