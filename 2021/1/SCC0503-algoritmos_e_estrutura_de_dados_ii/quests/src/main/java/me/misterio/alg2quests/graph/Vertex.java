package me.misterio.alg2quests.graph;

public class Vertex
{
    private int id;
    private String name;
    private String description;

    public Vertex(int id, String name, String description)
    {
        this.id = id;
        this.name = name;
        this.description = description;
    }

    public int getId()
    {
        return id;
    }

    @Override
    public String toString()
    {
        return "Quest{\n" +
                "\tID= '" + id + "'\n" +
                "\tname= '" + name + "'\n" +
                "\tdescription= '" + description + "'\n" +
                "}";
    }
}
