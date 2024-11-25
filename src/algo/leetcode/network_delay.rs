/// 743. 网络延迟时间
/// 有 n 个网络节点，标记为 1 到 n。
/// 给你一个列表 times，表示信号经过 有向 边的传递时间。 times[i] = (ui, vi, wi)，其中 ui 是源节点，vi 是目标节点， wi 是一个信号从源节点传递到目标节点的时间。
/// 现在，从某个节点 K 发出一个信号。需要多久才能使所有节点都收到信号？如果不能使所有节点收到信号，返回 -1 。
///
/// 示例 1：
/// 输入：times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
/// 输出：2
///
/// 示例 2：
/// 输入：times = [[1,2,1]], n = 2, k = 1
/// 输出：1
///
/// 示例 3：
/// 输入：times = [[1,2,1]], n = 2, k = 2
/// 输出：-1
///
/// 提示：
/// 1 <= k <= n <= 100
/// 1 <= times.length <= 6000
/// times[i].length == 3
/// 1 <= ui, vi <= n
/// ui != vi
/// 0 <= wi <= 100
/// 所有 (ui, vi) 对都 互不相同（即，不含重复边）
///
use crate::algo::graph::dijkstra;

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let edge_count = times.len();
    if (edge_count as i32) < n - 1 {
        return -1;
    }

    let n = n as usize;
    let mut graph = vec![vec![-1; n]; n];
    for i in 0..n {
        graph[i][i] = 0;
    }

    for edge in times {
        let (src, des, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
        graph[src - 1][des - 1] = weight;
    }

    let distance = dijkstra::dijkstra(&graph, (k - 1) as usize);
    let mut max_time = 0;
    for d in distance {
        if d == usize::MAX {
            return -1;
        }
        if d > max_time {
            max_time = d;
        }
    }
    return max_time as i32;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn network_delay_test() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        assert_eq!(network_delay_time(times, n, k), 2);

        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        assert_eq!(network_delay_time(times, n, k), 1);

        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        assert_eq!(network_delay_time(times, n, k), -1);

        let times = vec![
            vec![3, 5, 78],
            vec![2, 1, 1],
            vec![1, 3, 0],
            vec![4, 3, 59],
            vec![5, 3, 85],
            vec![5, 2, 22],
            vec![2, 4, 23],
            vec![1, 4, 43],
            vec![4, 5, 75],
            vec![5, 1, 15],
            vec![1, 5, 91],
            vec![4, 1, 16],
            vec![3, 2, 98],
            vec![3, 4, 22],
            vec![5, 4, 31],
            vec![1, 2, 0],
            vec![2, 5, 4],
            vec![4, 2, 51],
            vec![3, 1, 36],
            vec![2, 3, 59],
        ];
        let n = 5;
        let k = 5;
        assert_eq!(network_delay_time(times, n, k), 31);
    }
}
