import java.util.Scanner;
import java.util.HashMap;

class Solution {
        public static class UnionFind<T> {
        private HashMap<T, T> id = new HashMap<>();

        public T find(T x) {
            T y = id.getOrDefault(x, x);
            if (y != x) {
                y = find(y);
                id.put(x, y);
            }
            return y;
        }

        public void union(T x, T y) {
            id.put(find(x), find(y));
        }

    }
    
    public static class SetCounter {
        private UnionFind<Integer> dsu = new UnionFind<>();
        private HashMap<Integer, Integer> counts = new HashMap<>();
        
        public void merge(int x, int y) {
            if (dsu.find(x) != dsu.find(y)){
                int newSize = count(x) + count(y);
                dsu.union(x, y);
                counts.put(dsu.find(x), newSize);
            }
        }

        public int count(int x) {
            return counts.getOrDefault(dsu.find(x), 1);
        }
        
    }
}