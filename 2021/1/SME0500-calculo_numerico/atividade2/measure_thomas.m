#!/usr/bin/env octave

function time = measure_thomas(n)
    A = diag(rand(n,1)) + diag(rand(n-1,1),1) + diag(rand(n-1,1),-1);
    b = rand(n,1);
    pre_time = cputime();
    thomas(A,b);
    time = cputime()-pre_time;
endfunction
