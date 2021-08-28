function x = solve_lu(A, b)
    [L, U, P] = lu(A);
    y = L\(P*b);
    x = U\y;
endfunction
