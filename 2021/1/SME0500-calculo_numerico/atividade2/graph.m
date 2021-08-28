#!/usr/bin/env bash
N=[16,32,64,128,256,512,1024,2048,4096];

THOMAS=[measure_thomas(16),measure_thomas(32),measure_thomas(64),measure_thomas(128),measure_thomas(256),measure_thomas(512),measure_thomas(1024),measure_thomas(2048),measure_thomas(4096)];
GAUSS_NP=[measure_gauss_np(16),measure_gauss_np(32),measure_gauss_np(64),measure_gauss_np(128),measure_gauss_np(256),measure_gauss_np(512),measure_gauss_np(1024),measure_gauss_np(2048),measure_gauss_np(4096)];
MLDIVIDE=[measure_mldivide(16),measure_mldivide(32),measure_mldivide(64),measure_mldivide(128),measure_mldivide(256),measure_mldivide(512),measure_mldivide(1024),measure_mldivide(2048),measure_mldivide(4096)];
CHOL=[measure_chol(16),measure_chol(32),measure_chol(64),measure_chol(128),measure_chol(256),measure_chol(512),measure_chol(1024),measure_chol(2048),measure_chol(4096)];
LU=[measure_lu(16),measure_lu(32),measure_lu(64),measure_lu(128),measure_lu(256),measure_lu(512),measure_lu(1024),measure_lu(2048),measure_lu(4096)];

loglog(N,THOMAS,"r-h",N,MLDIVIDE,"g-+",N,GAUSS_NP,"b-o",N,CHOL,"m-s",N,LU,"c-s", "linewidth",3)

legend({'Thomas','Octave \','Gauss sem pivô','Cholesky','LU'},'Location','NorthWest')
