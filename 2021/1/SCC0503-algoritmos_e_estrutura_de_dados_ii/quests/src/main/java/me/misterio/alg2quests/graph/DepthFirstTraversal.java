package me.misterio.alg2quests.graph;

import java.util.ArrayList;
import java.util.Stack;

public final class DepthFirstTraversal extends TraversalStrategyInterface
{

    public DepthFirstTraversal(AbstractGraph graph)
    {
        super(graph);
    }

    @Override
    public void traverseGraph(Vertex source) {
        traverseInner(source);
        printPath();
    }

    void traverseInner(Vertex current)
    {
        int currentIndex = getGraph().getVertices().indexOf(current);

        // Marcar como visitado
        markVertexAsVisited(currentIndex);
        // Adicionar ao caminho
        addToPath(current);

        // Iterar pelos adjacentes, começando pelo primeiro
        var adjacentVertex = getGraph().getFirstConnectedVertex(current);
        while(adjacentVertex != null)
        {
            int adjacentVertexIndex = getGraph().getVertices().indexOf(adjacentVertex);
            // Caso não tenha sido visitado
            if(!hasVertexBeenVisited(adjacentVertexIndex))
            {
                // Chamar novamente, recursivo
                traverseInner(adjacentVertex);
            }
            // Próximo adjacente
            adjacentVertex = getGraph().getNextConnectedVertex(current, adjacentVertex);
        }
    }
}
