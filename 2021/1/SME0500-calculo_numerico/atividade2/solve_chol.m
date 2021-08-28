function x = solve_chol(A, b)
    R = chol(A*A');
    x = R\(R'\b);
endfunction
