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

        public int count() {
            return id.size();
        }
    }
    
    public static class SetCounter {
        private UnionFind<Integer> dsu = new UnionFind<>();
        
        public void merge(int x, int y) {
            dsu.union(x, y);
        }

        public int count(int x) {
            return dsu.count();
        }
    }