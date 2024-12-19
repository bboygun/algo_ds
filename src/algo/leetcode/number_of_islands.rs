// 200. 岛屿数量
// 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
// 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
// 此外，你可以假设该网格的四条边均被水包围。

// 示例 1：
// 输入：grid = [
//   ['1','1','1','1','0'],
//   ['1','1','0','1','0'],
//   ['1','1','0','0','0'],
//   ['0','0','0','0','0']
// ]
// 输出：1

// 示例 2：
// 输入：grid = [
//   ['1','1','0','0','0'],
//   ['1','1','0','0','0'],
//   ['0','0','1','0','0'],
//   ['0','0','0','1','1']
// ]
// 输出：3

// 提示：
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] 的值为 '0' 或 '1'

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut is_visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

    let mut cnt = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' && !is_visited[i][j] {
                cnt += 1;
                dfs(&grid, &mut is_visited, i, j);
            }
        }
    }
    cnt
}

fn dfs(grid: &Vec<Vec<char>>, is_visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let m = grid.len();
    let n = grid[0].len();

    if i == m || j == n || grid[i][j] == '0' || is_visited[i][j] {
        return;
    }

    is_visited[i][j] = true;
    dfs(grid, is_visited, i + 1, j);
    if (i > 0) {
        dfs(grid, is_visited, i - 1, j);
    }
    dfs(grid, is_visited, i, j + 1);
    if (j > 0) {
        dfs(grid, is_visited, i, j - 1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn num_islands_test() {
        let grid1 = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        let grid2 = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid1), 1);
        assert_eq!(num_islands(grid2), 3);
    }
}
