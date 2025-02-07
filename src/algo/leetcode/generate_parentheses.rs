// 22. 括号生成
// 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

// 示例 1：
// 输入：n = 3
// 输出：["((()))","(()())","(())()","()(())","()()()"]

// 示例 2：
// 输入：n = 1
// 输出：["()"]

// 提示：
// 1 <= n <= 8

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut track: Vec<char> = vec![];

    backtrace(n as usize, 0, 0, &mut result, &mut track);
    result
}

fn backtrace(
    n: usize,
    left_cnt: usize,
    right_cnt: usize,
    result: &mut Vec<String>,
    track: &mut Vec<char>,
) {
    if (right_cnt == n) {
        result.push(String::from_iter(track.clone()));
        return;
    }

    if left_cnt < n {
        track.push('(');
        backtrace(n, left_cnt + 1, right_cnt, result, track);
        track.pop();
    }

    if right_cnt < left_cnt {
        track.push(')');
        backtrace(n, left_cnt, right_cnt + 1, result, track);
        track.pop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_parentheses_test() {
        let n1 = 3;
        let ans1 = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(generate_parenthesis(n1), ans1);

        let n2 = 1;
        let ans2 = vec!["()"];
        assert_eq!(generate_parenthesis(n2), ans2);
    }
}
