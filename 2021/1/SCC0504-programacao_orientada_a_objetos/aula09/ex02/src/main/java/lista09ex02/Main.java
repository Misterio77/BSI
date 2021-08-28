package lista09ex02;

import java.util.Arrays;

import lista09ex02.Point;

public class Main {
    public static <T extends Comparable<T>> void insertionSort(T[] input) {
        int len = input.length;
        for (int i = 1; i < len; ++i) {
            T tmp = input[i];
            int j = i - 1;
            while (j >= 0 && input[j].compareTo(tmp) > 0){
                input[j+1] = input[j];
                j = j - 1;
            }
            input[j+1] = tmp;
        }
    };
    public static void main(String[] args) {
        System.out.println("Integers (letra a):");
        Integer[] ints = { 14, 5, 82, 3, 94, 5, 78, 2, 39, 4 };
        System.out.println("Desordenado: "+ Arrays.toString(ints));
        insertionSort(ints);
        System.out.println("Ordenado: "+ Arrays.toString(ints));

        System.out.println();

        System.out.println("Points (letra b e c):");
        Point[] points = {
            new Point(1,2),
            new Point(3,-1),
            new Point(10,2),
            new Point(-5,5),
            new Point(-10,1),
            new Point(0,0),
            new Point(9,8),
            new Point(9,1),
            new Point(2,3),
            new Point(4,5)
        };

        System.out.println("Desordenado: "+ Arrays.toString(points));
        insertionSort(points);
        System.out.println("Ordenado: "+ Arrays.toString(points));
    }
}
