N = [1 * 0.57119866428, 2 * 0.57119866428, 3 * 0.57119866428, 4 * 0.57119866428, 5 * 0.57119866428, 6 * 0.57119866428, 7 * 0.57119866428, 8 * 0.57119866428, 9 * 0.57119866428, 10 * 0.57119866428];

## 1 representacao
##k1 = [0.439359, 0.033571, -0.416101,-0.797445,-1.015299,-1.015299,-0.797445,-0.416101,0.033571,0.439359];
##k2 = [0.605607, 0.323343, -0.064362,-0.425159,-0.637336,-0.637336,-0.425159,-0.064362, 0.323343, 0.605607];
##k3 = [0.5747, 0.2719,-0.1186,-0.4705,-0.6743,-0.6743,-0.4705,-0.1186, 0.2719, 0.5747];
##k4 = [0.5743, 0.2714,-0.1190,-0.4706,-0.6744,-0.6744,-0.4706,-0.1190, 0.2714, 0.5743];
##k5 = [0.5743, 0.2714,-0.1190,-0.4706,-0.6744,-0.6744,-0.4706,-0.1190, 0.2714, 0.5743];
##k6 = [0.5743, 0.2714,-0.1190,-0.4706,-0.6744,-0.6744,-0.4706,-0.1190, 0.2714, 0.5743];
##k7 = [0.5743, 0.2714,-0.1190,-0.4706,-0.6744,-0.6744,-0.4706,-0.1190, 0.2714, 0.5743];


## 2 representacao
k1 = [2.8572, 4.4032, 5.3060, 5.7405, 5.9013, 5.9013, 5.7405, 5.3060, 4.4032, 2.8572];
k2 = [2.8291, 4.8579, 7.2426, 9.5443,10.9518,10.9518, 9.5443, 7.2426, 4.8579, 2.8291];
k3 = [1.6022,2.0231,2.9010,4.3248,4.0966,4.0966,4.3248,2.9010,2.0231,1.6022];
k4 = [1.5604,2.0944,2.3450,2.3416,2.3958,2.3958,2.3416,2.3450,2.0944,1.5604];
k5 = [1.6511,2.2757,2.6473,2.8546,2.9445,2.9445,2.8546,2.6473,2.2757,1.6511];
k6 = [1.6378,2.2501,2.6084,2.8008,2.8841,2.8841,2.8008,2.6084,2.2501,1.6378];
k7 = [1.6377,2.2499,2.6081,2.8005,2.8837,2.8837,2.8005,2.6081,2.2499,1.6377];

hold on;
plot(N, k1);
plot(N, k2);
plot(N, k3);
plot(N, k4);
plot(N, k5);
plot(N, k6);
plot(N, k7);


xlabel('Time','fontsize',14);
ylabel('Theta(i)','fontsize',14);
legend({'k = 1','k = 2', 'k = 3', 'k = 4', 'k = 5', 'k = 6', 'k = 7'}, 'Location','NorthWest');
set(gca,'fontsize',14);

hold off;