package lista09ex03;

import lista09ex03.Node;
import lista09ex03.BinaryTree;

public class Main {
    public static void main(String[] args) {
        // Aqui fazemos uma btree com Integer, mas funciona
        // com qqr tipo que extende Comparable
        var bt = new BinaryTree<Integer>();

        System.out.println("Adicionando 10, 5, 4, -4, 8:");
        bt.add(10);
        bt.add(5);
        bt.add(4);
        bt.add(-4);
        bt.add(8);
        bt.traverseInOrder(bt.root);
        System.out.println();

        System.out.println();

        System.out.println("Removendo 5:");
        bt.delete(5);
        bt.traverseInOrder(bt.root);
        System.out.println();
    }
}
