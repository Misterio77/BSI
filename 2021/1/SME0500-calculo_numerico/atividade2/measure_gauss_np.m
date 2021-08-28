#!/usr/bin/env octave

function time = measure_gauss_np(n)
    A = diag(rand(n,1)) + diag(rand(n-1,1),1) + diag(rand(n-1,1),-1);
    b = rand(n,1);
    pre_time = cputime();
    gauss_np(A,b);
    time = cputime()-pre_time;
endfunction
