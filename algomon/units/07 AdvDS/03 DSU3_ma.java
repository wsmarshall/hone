//implemented here in Java, loop back around for "Class" impl 
//in Rust

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

class Solution {
    public static List<List<String>> mergeAccounts(List<List<String>> accounts) {
        // WRITE YOUR BRILLIANT CODE HERE
        return List.of();
    }

    public static List<String> splitWords(String s) {
        return s.isEmpty() ? List.of() : Arrays.asList(s.split(" "));
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int accountsLength = Integer.parseInt(scanner.nextLine());
        List<List<String>> accounts = new ArrayList<>();
        for (int i = 0; i < accountsLength; i++) {
            accounts.add(splitWords(scanner.nextLine()));
        }
        scanner.close();
        List<List<String>> res = mergeAccounts(accounts);
        for (List<String> row : res) {
            System.out.println(String.join(" ", row));
        }
    }
}