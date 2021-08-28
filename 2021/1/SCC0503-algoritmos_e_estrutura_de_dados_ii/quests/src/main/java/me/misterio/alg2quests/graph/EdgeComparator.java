package me.misterio.alg2quests.graph;

import java.util.Comparator;
public class EdgeComparator implements Comparator<Edge>
{
    public int compare(Edge v1, Edge v2)
    {
        return v1.getDestination().getId() - v2.getDestination().getId();
    }
}
