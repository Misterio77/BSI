## Copyright (C) 2021 jatra
##
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
##
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
##
## You should have received a copy of the GNU General Public License
## along with this program.  If not, see <https://www.gnu.org/licenses/>.

function [x,k] = newton_sis(F, Jac, x, tol, kmax)
% F : funcao vetorial
% Jac : Jacobiano de F
% x : chute inicial (vetor coluna)

  if nargin == 4
    kmax = 1000;
  end
  
  for k = 1 : kmax
    v = Jac(x) \ F(x);
    x = x - v;
    
    if norm(v) < tol
      return;
    end
  end

endfunction