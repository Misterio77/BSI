package me.misterio.alg2brabalog;

import me.misterio.alg2brabalog.graph.Vertex;
import me.misterio.alg2brabalog.graph.DigraphMatrix;
import me.misterio.alg2brabalog.graph.FloydWarshallTraversal;

import java.util.Scanner;
import java.util.Arrays;
import java.util.ArrayList;
import java.util.List;

public class Main {
    /** Encontrar vértice numa lista de vértices, dado seu x e y
     *
     * A igualdade entre os objetos não funciona, então se criarmos um vertice
     * com os mesmos dados, ele não encontrará seu correspondente, quando for
     * chamada a função addEdge
     */
    public static Vertex findVertex(final List<Vertex> list, final Integer x, final Integer y) {
        final var vertex = list.stream().filter(o ->
            o.equals(new Vertex (x, y))
        ).findFirst().get();

        return vertex;
    }

    public static void main(String[] args) throws Exception {
        // Dêem uma olhada na README.md para info de quais arquivos modifiquei

        var sc = new Scanner(System.in);

        var vertices = new ArrayList<Vertex>();

        // Número de vértices
        final var nv = Integer.parseInt(sc.nextLine());
        // Vértices
        for (var i = 0; i < nv; i++) {
            final var line = sc.nextLine().split(",");

            final var x = Integer.parseInt(line[0]);
            final var y = Integer.parseInt(line[1]);

            final var vertex = new Vertex(x, y);
            // Adicionar vértice
            vertices.add(vertex);
        }

        // Criar grafo com nossos vértices
        var graph = new DigraphMatrix(vertices);

        // Número de arestas
        final var ne = Integer.parseInt(sc.nextLine());
        // Arestas
        for (var i = 0; i < ne; i++) {
            // Cortar no :
            final var input = sc.nextLine().split(":");

            // Vértice origem
            final var srcInput = input[0].split(",");
            final var srcX = Integer.parseInt(srcInput[0]);
            final var srcY = Integer.parseInt(srcInput[1]);
            final var src = findVertex(vertices, srcX, srcY);

            // Vértice destino
            final var dstInput = input[1].split(",");
            final var dstX = Integer.parseInt(dstInput[0]);
            final var dstY = Integer.parseInt(dstInput[1]);
            final var dst = findVertex(vertices, dstX, dstY);

            // O peso será a distância euclideana entre os dois
            final var weight = src.euclideanDistance(dst);
            // Adicionar aresta
            graph.addEdge(src, dst, weight);
        }

        // Criar e chamar o floyd-warshall
        var traversal = new FloydWarshallTraversal(graph);
        traversal.traverseGraph(null);

        // Encontrar o vértice mais central
        var central = traversal.mostCentral();
        System.out.println(central.getX()+","+central.getY());

        // Encontrar o vértice mais periférico
        var peripheric = traversal.mostPeripheric();
        System.out.println(peripheric.getX()+","+peripheric.getY());
        
        // Encontrar o vértice mais distance do vértice mais periférico
        var farthest = traversal.farthestFrom(peripheric);
        System.out.println(farthest.getX()+","+farthest.getY());
    }
}
