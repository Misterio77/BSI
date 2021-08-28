package lista09ex03;

public class Node<T extends Comparable<T>> {
    T data;
    Node<T> left;
    Node<T> right;

    Node(T data) {
        this.data = data;
        left = null;
        right = null;
    }

}
