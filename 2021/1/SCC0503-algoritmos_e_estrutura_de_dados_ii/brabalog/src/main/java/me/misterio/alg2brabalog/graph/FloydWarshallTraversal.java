package me.misterio.alg2brabalog.graph;

import me.misterio.alg2brabalog.graph.TraversalStrategyInterface;
import java.text.DecimalFormat;
import java.util.logging.Logger;
import java.util.Arrays;

public class FloydWarshallTraversal extends TraversalStrategyInterface
{
    private static final Logger LOGGER = Logger.getLogger("FloydWarshallTraversal.class");

    private float [][]distanceMatrix;

    public float[][] getDistanceMatrix()
    {
        return distanceMatrix;
    }

    public void setDistanceMatrix(float[][] distanceMatrix)
    {
        this.distanceMatrix = distanceMatrix;
    }

    public FloydWarshallTraversal(AbstractGraph graph)
    {
        super(graph);
        distanceMatrix = new float[graph.getNumberOfVertices()][graph.getNumberOfVertices()];
    }

    @Override
    public void traverseGraph(Vertex source)
    {
        for (var i = 0; i < getGraph().getNumberOfVertices(); i++)
        {
            for (var j = 0; j < getGraph().getNumberOfVertices(); j++)
            {
                Vertex origin = getGraph().getVertices().get(i);
                Vertex destination = getGraph().getVertices().get(j);
                distanceMatrix[i][j] = getGraph().getDistance(origin, destination);
            }
        }

        for (var k = 0; k < getGraph().getNumberOfVertices(); k++)
        {
            for (var i = 0; i < getGraph().getNumberOfVertices(); i++)
            {
                for (var j = 0; j < getGraph().getNumberOfVertices(); j++)
                {
                    if((distanceMatrix[i][k] + distanceMatrix[k][j]) < distanceMatrix[i][j])
                    {
                        distanceMatrix[i][j] = distanceMatrix[i][k] + distanceMatrix[k][j];
                    }
                }
            }
        }
    }

    /**Encontra o vértice mais central, usando dados da travessia
     */
    public Vertex mostCentral()
    {
        // Número de vértices do grafo
        final var verticesNum = getGraph().getNumberOfVertices();

        // Índice e distância inicial
        var centralIndex = 0;
        var centralDistances = Float.POSITIVE_INFINITY;

        // Percorrer todos os vértices
        for (var i = 0; i < verticesNum; i++)
        {
            // Começar em 0
            var distances = (float) 0.0;
            // Para cada vértice, pegar e somar distancia
            for (var j = 0; j < verticesNum; j++)
            {
                distances += distanceMatrix[i][j];
            }
            // Caso a soma de distância desses seja menor que a armazenada
            if (distances < centralDistances) {
                centralDistances = distances;
                centralIndex = i;
            }
        }
        return getGraph().getVertices().get(centralIndex);
    }

    /**Encontra o vértice mais periférico, usando dados da travessia
     */
    public Vertex mostPeripheric()
    {
        // Número de vértices do grafo
        final var verticesNum = getGraph().getNumberOfVertices();

        // Índice e distância inicial
        var periphericIndex = 0;
        var periphericDistances = Float.NEGATIVE_INFINITY;

        // Percorrer todos os vértices
        for (var i = 0; i < verticesNum; i++)
        {
            // Começar em 0
            var distances = (float) 0.0;
            // Para cada vértice, pegar e somar distancia
            for (var j = 0; j < verticesNum; j++)
            {
                if (j != i) distances += distanceMatrix[i][j];
            }
            // Caso a soma de distância desses seja maior que a armazenada
            if (distances > periphericDistances) {
                periphericDistances = distances;
                periphericIndex = i;
            }
        }
        return getGraph().getVertices().get(periphericIndex);
    }

    /**Dado um vértice, encontra o vertíce mais distante dele, usando dados da travessia
     */
    public Vertex farthestFrom(Vertex source)
    {
        // Pegar índice da origem
        var sourceIndex = getGraph().getVertices().indexOf(source);
        // Número de vértices no grafo
        final var verticesNum = getGraph().getNumberOfVertices();

        // Indice e distancia inicial
        var farthestIndex = sourceIndex;
        var farthestDistance = Double.NEGATIVE_INFINITY;
        // Percorrer todos os vértices
        for (var i = 0; i < verticesNum; i++)
        {
            // Distância do dado para este
            var distance = distanceMatrix[sourceIndex][i];
            // Caso seja maior que a que temos armazenada
            if (i != sourceIndex && distance > farthestDistance) {
                farthestDistance = distance;
                farthestIndex = i;
            }
        }

        return getGraph().getVertices().get(farthestIndex);
    }

}
