package me.misterio.alg2brabalog.graph;

import java.lang.Math;

public class Vertex
{
    private Integer x;
    private Integer y;

    public Vertex(Integer x, Integer y)
    {
        this.x = x;
        this.y = y;
    }

    public Integer getX()
    {
        return x;
    }
    public Integer getY()
    {
        return y;
    }

    @Override
    public String toString()
    {
        return "Vertex{(" + x + "," + y + ")}";
    }

    public float euclideanDistance(Vertex other)
    {
        return ((float) Math.sqrt(Math.pow(other.x - this.x, 2)+Math.pow(other.y - this.y, 2)));
    }

    public boolean equals(Vertex other) {
        if (this.x == other.x && this.y == other.y) return true;
        return false;
    }
}
