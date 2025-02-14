// 131. 分割回文串
// 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文串。返回 s 所有可能的分割方案。

// 示例 1：
// 输入：s = "aab"
// 输出：[["a","a","b"],["aa","b"]]

// 示例 2：
// 输入：s = "a"
// 输出：[["a"]]

// 提示：
// 1 <= s.length <= 16
// s 仅由小写英文字母组成

fn partition(s: String) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn palindrome_partitioning_test() {
        let s1 = "aab";
        let ans1 = vec![vec!["a", "a", "b"], vec!["b"]];
        assert_eq!(partition(s1.into()), ans1);

        let s2 = "a";
        let ans2 = vec![vec!["a"]];
        assert_eq!(partition(s2.into()), ans2);
    }
}
