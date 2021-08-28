function time = measure_chol(n)
    A = diag(rand(n,1)) + diag(rand(n-1,1),1) + diag(rand(n-1,1),-1);
    b = rand(n,1);
    pre_time = cputime();
    solve_chol(A,b);
    time = cputime()-pre_time;
endfunction
