package me.misterio.alg2quests.graph;

import java.util.LinkedList;
import java.util.Queue;

public final class BreadthFirstTraversal extends TraversalStrategyInterface
{

    public BreadthFirstTraversal(AbstractGraph graph)
    {
        super(graph);
    }

    @Override
    public void traverseGraph(Vertex source)
    {
        int sourceIndex = getGraph().getVertices().indexOf(source);
        addToPath(source);
        markVertexAsVisited(sourceIndex);

        Queue<Vertex> vertexesToVisit = new LinkedList<>();
        vertexesToVisit.add(source);

        Vertex currentVisitedVertex;
        int currentVisitedVertexIndex;

        while(!vertexesToVisit.isEmpty())
        {
            currentVisitedVertex = vertexesToVisit.poll();
            if (currentVisitedVertex != null)
            {
                currentVisitedVertexIndex = getGraph().getVertices().indexOf(currentVisitedVertex);
                var adjacentVertex = getGraph().getFirstConnectedVertex(currentVisitedVertex);
                while(adjacentVertex != null)
                {
                    int adjacentVertexIndex = getGraph().getVertices().indexOf(adjacentVertex);
                    if(!hasVertexBeenVisited(adjacentVertexIndex))
                    {
                        addToPath(adjacentVertex);
                        markVertexAsVisited(adjacentVertexIndex);
                        vertexesToVisit.add(adjacentVertex);
                    }
                    adjacentVertex = getGraph().getNextConnectedVertex(currentVisitedVertex, adjacentVertex);
                }
            }
        }
        printPath();
    }
}
