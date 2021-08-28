package me.misterio.alg2quests;

import me.misterio.alg2quests.graph.Vertex;
import me.misterio.alg2quests.graph.DigraphList;
import me.misterio.alg2quests.graph.BreadthFirstTraversal;
import me.misterio.alg2quests.graph.DepthFirstTraversal;

import java.util.Scanner;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) throws Exception {
        // Nota: Dêem uma olhadinha no README.md para um sumário do que eu mudei no código de vocês

        // Instanciar scanner
        var sc = new Scanner(System.in);

        // Guardar vertices lidos
        var vertices = new ArrayList<Vertex>();

        // Ler número de vértices
        var nv = Integer.parseInt(sc.nextLine());
        for (int i = 0; i < nv; i++) {
            var name = sc.nextLine();
            var description = sc.nextLine();
            // Criar vértice com as infos digitadas
            var quest = new Vertex(i, name, description);
            // Aicionar a lista
            vertices.add(quest);
        }

        // Criar um grafo com os vértices
        // Optei por usar DiagraphList, já que não vamos ter que remover coisas do grafo
        var graph = new DigraphList(vertices);

        // Ler número de arestas
        var na = Integer.parseInt(sc.nextLine());
        for (var i = 0; i < na; i++) {
            // A linha é 'fonte destino'
            // Então vamos ler ela inteira, e cortar com espaço de delimitador
            var tmp = sc.nextLine().split(" ");
            // Parsear primeiro token
            var src = Integer.parseInt(tmp[0]);
            // Parsear segundo token
            var dst = Integer.parseInt(tmp[1]);
            // Adicionar vértice
            graph.addEdge(vertices.get(src), vertices.get(dst), 1);
        }

        // Ler índice do vetor inicial, e pegar ele
        var source = vertices.get(Integer.parseInt(sc.nextLine()));
        // Fechar scanner
        sc.close();

        // Instanciar travessia
        var traversal = new DepthFirstTraversal(graph);
        // Fazer a travessia
        traversal.traverseGraph(source);
    }
}
