use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn f(bitmask: usize, current_node: usize, graph: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if bitmask == ((1 << graph.len()) - 1) {
        return 0;
    }

    if dp[bitmask][current_node] != i32::MAX {
        return dp[bitmask][current_node];
    }

    let mut ans = i32::MAX;
    for i in 0..graph[current_node].len() {
        if (bitmask & (1 << i)) == 0 && graph[current_node][i] != 0 {
            ans = cmp::min(
                ans,
                graph[current_node][i] + f((bitmask | (1 << i)), i, graph, dp),
            );
        }
    }

    dp[bitmask][current_node] = ans;
    ans
}

fn min_cost_to_visit_every_node(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let base: i32 = 2;
    let mut dp = vec![vec![i32::MAX; n]; base.pow(n.try_into().unwrap()) as usize];

    let ans = f(1, 0, &graph, &mut dp);
    if ans == i32::MAX {
        return -1;
    }
    ans
}
