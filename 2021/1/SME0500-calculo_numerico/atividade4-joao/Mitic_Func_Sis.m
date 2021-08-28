function [y] = Func_Sis(x)
     y(1) = 4*x(1) - x(2) + x(3) - x(1)*x(4);
     y(2) = - x(1) + 3*x(2) - 2*x(3) - x(2)*x(4);
     y(3) = x(1) - 2*x(2) + 3*x(3) - x(3)*x(4);
     y(4) = (x(1))^2 + (x(2))^2 + (x(3))^2 - 1;
     return
end


