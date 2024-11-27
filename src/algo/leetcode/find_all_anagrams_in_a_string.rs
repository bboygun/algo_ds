// 438. 找到字符串中所有字母异位词
// 给定两个字符串 s 和 p，找到 s 中所有 p 的异位词的子串，返回这些子串的起始索引。不考虑答案输出的顺序。

// 示例 1:
// 输入: s = "cbaebabacd", p = "abc"
// 输出: [0,6]
// 解释:
// 起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
// 起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。

// 示例 2:
// 输入: s = "abab", p = "ab"
// 输出: [0,1,2]
// 解释:
// 起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
// 起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
// 起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。

// 提示:
// 1 <= s.length, p.length <= 3 * 10^4
// s 和 p 仅包含小写字母

use std::f32::consts::PI;

fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if s.len() < p.len() {
        return vec![];
    }

    let mut p_count = vec![0; 26];
    for c in p.chars() {
        let to_add = c as usize - 'a' as usize;
        p_count[to_add] += 1;
    }

    let mut window_count = vec![0; 26];
    let chars: Vec<char> = s.chars().collect();

    let mut ans = vec![];
    let (mut left, mut right) = (0, 0);
    while right < s.len() {
        // 右指针滑动到窗口右边，给经过的字符计数
        while right - left < p.len() {
            let to_add = chars[right] as usize - 'a' as usize;
            window_count[to_add] += 1;
            right += 1;
        }

        // 此时窗口和p一样大，进行比较
        if window_count == p_count {
            ans.push(left as i32);
        }

        // 左指针向左
        let to_delete = chars[left] as usize - 'a' as usize;
        window_count[to_delete] -= 1;
        left += 1;
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_anagrams_test() {
        assert_eq!(find_anagrams("cbaebabacd".into(), "abc".into()), vec![0, 6]);
        assert_eq!(find_anagrams("abab".into(), "ab".into()), vec![0, 1, 2]);
        assert_eq!(find_anagrams("a".into(), "ab".into()), vec![]);
        assert_eq!(find_anagrams("abc".into(), "cab".into()), vec![0]);
        assert_eq!(find_anagrams("aaa".into(), "a".into()), vec![0, 1, 2]);
    }
}
