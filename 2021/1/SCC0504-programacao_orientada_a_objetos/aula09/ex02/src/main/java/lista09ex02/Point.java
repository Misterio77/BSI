package lista09ex02;

import java.lang.Comparable;

public class Point implements Comparable<Point> {
    private Integer x;
    private Integer y;
    Point(Integer x, Integer y) {
        this.x = x;
        this.y = y;
    }

    public int compareTo(Point other) {
        int comparision = this.x.compareTo(other.x);
        if (comparision != 0) {
            return comparision;
        } else {
            return this.y.compareTo(other.y);
        }
    }

    public String toString() {
        return String.format("(" + this.x + ", " + this.y + ")");
    }

}
