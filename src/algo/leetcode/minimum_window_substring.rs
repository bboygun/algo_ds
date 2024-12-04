// 76. 最小覆盖子串
// 提示
// 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。

// 注意：
// 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
// 如果 s 中存在这样的子串，我们保证它是唯一的答案。

// 示例 1：
// 输入：s = "ADOBECODEBANC", t = "ABC"
// 输出："BANC"
// 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。

// 示例 2：
// 输入：s = "a", t = "a"
// 输出："a"
// 解释：整个字符串 s 是最小覆盖子串。

// 示例 3:
// 输入: s = "a", t = "aa"
// 输出: ""
// 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
// 因此没有符合条件的子字符串，返回空字符串。

// 提示：
// m == s.length
// n == t.length
// 1 <= m, n <= 10^5
// s 和 t 由英文字母组成

// 进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？

fn min_window(s: String, t: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let mut t_count = vec![0; 58];
    for char in t_chars {
        let index = char as usize - 'A' as usize;
        t_count[index] += 1;
    }

    let mut sub_count = vec![0; 58];
    let (mut left, mut right) = (0, 0);
    let (mut ans_left, mut ans_right) = (-1, -1);
    let mut length = usize::MAX;
    while right < s.len() {
        let index = s_chars[right] as usize - 'A' as usize;
        sub_count[index] += 1;
        while cover(&sub_count, &t_count) {
            if right - left + 1 < length {
                length = right - left + 1;
                ans_left = left as i32;
                ans_right = right as i32;
            }
            let delete_index = s_chars[left] as usize - 'A' as usize;
            sub_count[delete_index] -= 1;
            left += 1;
        }
        right += 1;
    }
    if ans_right == -1 {
        return String::new();
    }
    String::from_iter(s_chars[ans_left as usize..=ans_right as usize].iter())
}

fn cover(sub_count: &Vec<i32>, t_count: &Vec<i32>) -> bool {
    assert_eq!(sub_count.len(), t_count.len());
    for (&cur_count, &target_count) in sub_count.iter().zip(t_count.iter()) {
        if target_count > 0 && cur_count < target_count {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_windows_test() {
        assert_eq!(
            min_window("ADOBECODEBANC".into(), "ABC".into()),
            "BANC".to_string()
        );
        assert_eq!(min_window("a".into(), "a".into()), "a".to_string());
        assert_eq!(min_window("a".into(), "aa".into()), "".to_owned());
    }
}
