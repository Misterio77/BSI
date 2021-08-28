function [x n status] = Newton_Sis(x,tol,nmax)
   n = 0;
   erro = 2*tol*norm(x);
   
   while n < nmax && erro >= tol*norm(x)
       f  = Func_Sis(x);
       df = DFunc_Sis(x);
       y  = -df\f';
       erro = norm(y);
       x    = x+y;
       n    = n+1;
   end
   
   if n < nmax
       status = 1;
   else
       status = -1;
   end
   
   return

end

