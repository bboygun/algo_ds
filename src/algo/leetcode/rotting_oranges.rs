// 994. 腐烂的橘子
// 在给定的 m x n 网格 grid 中，每个单元格可以有以下三个值之一：
// 值 0 代表空单元格；
// 值 1 代表新鲜橘子；
// 值 2 代表腐烂的橘子。
// 每分钟，腐烂的橘子 周围 4 个方向上相邻 的新鲜橘子都会腐烂。
// 返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，返回 -1 。

// 示例 1：
// 输入：grid = [[2,1,1],[1,1,0],[0,1,1]]
// 输出：4

// 示例 2：
// 输入：grid = [[2,1,1],[0,1,1],[1,0,1]]
// 输出：-1
// 解释：左下角的橘子（第 2 行， 第 0 列）永远不会腐烂，因为腐烂只会发生在 4 个方向上。

// 示例 3：
// 输入：grid = [[0,2]]
// 输出：0
// 解释：因为 0 分钟时已经没有新鲜橘子了，所以答案就是 0 。

// 提示：
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 10
// grid[i][j] 仅为 0、1 或 2

fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let m = grid.len() as isize;
    let n = grid[0].len() as isize;

    let mut round = vec![];
    let mut fresh_cnt = 0;
    for i in 0..m {
        for j in 0..n {
            match grid[i as usize][j as usize] {
                1 => fresh_cnt += 1,
                2 => round.push((i, j)),
                _ => continue,
            }
        }
    }

    let direction = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    let mut time = 0;
    while !round.is_empty() && fresh_cnt > 0 {
        let mut next_round = vec![];
        while !round.is_empty() {
            let (i, j) = round.pop().unwrap();
            for d in direction.iter() {
                let (new_i, new_j) = (i + d[0], j + d[1]);
                if (new_i >= 0
                    && new_i < m
                    && new_j >= 0
                    && new_j < n
                    && grid[new_i as usize][new_j as usize] == 1)
                {
                    grid[new_i as usize][new_j as usize] = 2;
                    next_round.push((new_i, new_j));
                    fresh_cnt -= 1;
                }
            }
        }
        round = next_round;
        time += 1;
    }

    match fresh_cnt {
        0 => time,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn oranges_rotting_test() {
        let grid1 = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let grid2 = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        let grid3 = vec![vec![0, 2]];

        assert_eq!(oranges_rotting(grid1), 4);
        assert_eq!(oranges_rotting(grid2), -1);
        assert_eq!(oranges_rotting(grid3), 0);
    }
}
