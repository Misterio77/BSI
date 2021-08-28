package me.misterio.alg2quests.graph;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.logging.Logger;

public abstract class TraversalStrategyInterface
{
    private AbstractGraph graph;
    private boolean[] visitedVertices;
    private List<Vertex> traversalPath;

    public void markVertexAsVisited(int vertexIndex)
    {
        visitedVertices[vertexIndex] = true;
    }

    public boolean hasVertexBeenVisited(int vertexIndex)
    {
        return visitedVertices[vertexIndex];
    }

    protected TraversalStrategyInterface(AbstractGraph graph)
    {
        this.graph = graph;
        visitedVertices = new boolean[graph.getNumberOfVertices()];
        Arrays.fill(visitedVertices, false);
        traversalPath = new LinkedList<>();
    }

    abstract void traverseGraph(Vertex source);

    public AbstractGraph getGraph()
    {
        return graph;
    }

    public void setGraph(AbstractGraph graph)
    {
        this.graph = graph;
    }

    protected void printPath()
    {
        var visitedPath = new StringBuilder();
        for (Vertex vertex : traversalPath)
        {
            System.out.println(vertex);
            System.out.println();
        }
        System.out.println();
    }

    public void addToPath(Vertex vertex)
    {
        traversalPath.add(vertex);
    }
}
