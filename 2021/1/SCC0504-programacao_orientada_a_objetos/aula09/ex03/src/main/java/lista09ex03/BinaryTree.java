package lista09ex03;

import lista09ex03.Node;

public class BinaryTree<T extends Comparable<T>> {
    Node<T> root;

    private Node<T> addRecursive(Node<T> current, T data) {
        if (current == null) {
            return new Node<T>(data);
        }

        if (current.data.compareTo(data) > 0) {
            current.left = addRecursive(current.left, data);
        } else if (data.compareTo(current.data) > 0) {
            current.right = addRecursive(current.right, data);
        } else {
            return current;
        }

        return current;
    }

    public void add(T data) {
        root = addRecursive(root, data);
    }

    private boolean containsNodeRecursive(Node<T> current, T data) {
        if (current == null) {
            return false;
        }

        int comparision = current.data.compareTo(data);
        if (comparision == 0) {
            return true;
        }

        if (comparision > 0) {
            return containsNodeRecursive(current.left, data);
        } else {
            return containsNodeRecursive(current.right, data);
        }
    }

    public boolean containsNode(T data) {
        return containsNodeRecursive(root, data);
    }

    private T findSmallestValue(Node<T> root) {
        return root.left == null ? root.data : findSmallestValue(root.left);
    }

    private Node<T> deleteRecursive(Node<T> current, T data) {
        if (current == null) {
            return null;
        }

        int comparision = current.data.compareTo(data);
        if (comparision == 0) {
            if (current.left == null && current.right == null) {
                return null;
            } else if (current.left == null) {
                return current.right;
            } else if (current.right == null) {
                return current.left;
            }

            T smallest = findSmallestValue(current.right);
            current.data = smallest;
            current.right = deleteRecursive(current.right, smallest);
            return current;
        }

        if (comparision > 0) {
            current.left = deleteRecursive(current.left, data);
            return current;
        } else {
            current.right = deleteRecursive(current.right, data);
            return current;
        }
    }

    public void delete(T data) {
        root = deleteRecursive(root, data);
    }

    public void traverseInOrder(Node<T> node) {
        if (node != null) {
            traverseInOrder(node.left);
            System.out.print(" " + node.data);
            traverseInOrder(node.right);
        }
    }

}
