package me.misterio.alg2brabalog.graph;

import me.misterio.alg2brabalog.graph.AbstractGraph;
import java.io.File;
import java.io.IOException;
import java.util.List;
import java.util.logging.Level;
import java.util.logging.Logger;

public class DigraphMatrix extends AbstractGraph
{
    private Edge[][] adjacencyMatrix;

    private static final Logger LOGGER = Logger.getLogger("DigraphMatrix.class");

    public DigraphMatrix(List<Vertex> vertices)
    {
        super(vertices);
        initializeAdjacencyMatrix();
    }

    private void initializeAdjacencyMatrix()
    {
        setAdjacencyMatrix(new Edge[getNumberOfVertices()][getNumberOfVertices()]);
        for (var i = 0; i < getNumberOfVertices(); i++)
        {
            for (var j = 0; j < getNumberOfVertices(); j++)
            {
                getAdjacencyMatrix()[i][j] = null;
            }
        }
    }

    @Override
    public void addVertex(Vertex vertex)
    {
        throw new UnsupportedOperationException();
    }

    @Override
    public void removeVertex(Vertex vertex)
    {
        throw new UnsupportedOperationException();
    }

    @Override
    public void addEdge(Vertex source, Vertex destination, float weight)
    {
        if(!edgeExists(source, destination))
        {
            getAdjacencyMatrix()[getVertices().indexOf(source)][getVertices().indexOf(destination)] = new Edge(weight);
        }
    }

    @Override
    public void removeEdge(Vertex source, Vertex destination)
    {
        int sourceIndex = getVertices().indexOf(source);
        int destinationIndex = getVertices().indexOf(destination);
        if(edgeExists(source, destination))
        {
            getAdjacencyMatrix()[sourceIndex][destinationIndex] = null;
        }
    }

    @Override
    public boolean edgeExists(Vertex source, Vertex destination)
    {
        int sourceIndex = getVertices().indexOf(source);
        int destinationIndex = getVertices().indexOf(destination);
        return getAdjacencyMatrix()[sourceIndex][destinationIndex] != null;
    }

    @Override
    public boolean hasAnyEdge(Vertex vertex)
    {
        int vertexIndex = getVertices().indexOf(vertex);
        for (var i = 0; i < getNumberOfVertices(); i++)
        {
            if(getAdjacencyMatrix()[vertexIndex][i] != null)
            {
                return true;
            }
        }
        return false;
    }

    @Override
    public Vertex getFirstConnectedVertex(Vertex vertex)
    {
        if(!hasAnyEdge(vertex))
        {
            return null;
        }
        else
        {
            var currentVertexIndex = 0;
            Vertex connected;
            do
            {
                connected = getVertices().get(currentVertexIndex++);
            }while(!edgeExists(vertex, connected));
            return connected;
        }
    }

    @Override
    public Vertex getNextConnectedVertex(Vertex source, Vertex currentConnection)
    {
        Vertex newConnection;
        for (int i = getVertices().indexOf(currentConnection)+1; i < getNumberOfVertices(); i++)
        {
            newConnection = getVertices().get(i);
            if(edgeExists(source, newConnection))
            {
                return newConnection;
            }
        }
        return null;
    }

    @Override
    public String toString() {
        var s = new StringBuilder();
        for (var i = 0; i < getNumberOfVertices(); i++)
        {
            s.append(i).append(": ");
            for (var j = 0; j < getNumberOfVertices(); ++j)
            {
                if(edgeExists(getVertices().get(i), getVertices().get(j)))
                {
                    s.append(getAdjacencyMatrix()[i][j].getWeight()).append(" ");
                }
                else
                {
                    s.append(0.0 + " ");
                }
            }
            s.append("\n");
        }
        return s.toString();
    }

    @Override
    public float getDistance(Vertex source, Vertex destination)
    {
        var edge = adjacencyMatrix[getVertices().indexOf(source)][getVertices().indexOf(destination)];
        if(edge != null)
        {
            return edge.getWeight();
        }
        else
        {
            return Float.POSITIVE_INFINITY;
        }
    }

    public Edge[][] getAdjacencyMatrix()
    {
        return adjacencyMatrix;
    }

    public void setAdjacencyMatrix(Edge[][] adjacencyMatrix)
    {
        this.adjacencyMatrix = adjacencyMatrix;
    }
}
