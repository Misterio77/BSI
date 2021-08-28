function time = measure_lu(n)
    A = diag(rand(n,1)) + diag(rand(n-1,1),1) + diag(rand(n-1,1),-1);
    b = rand(n,1);
    pre_time = cputime();
    x = solve_lu(A, b);
    time = cputime()-pre_time;
endfunction
