// 3. 无重复字符的最长子串
// 给定一个字符串s，请你找出其中不含有重复字符的最长子串的长度。

// 示例 1:
// 输入: s = "abcabcbb"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。

// 示例 2:
// 输入: s = "bbbbb"
// 输出: 1
// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。

// 示例 3:
// 输入: s = "pwwkew"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

// 提示：
// 0 <= s.length <= 5 * 104
// s 由英文字母、数字、符号和空格组成

use std::collections::HashMap;

fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_len = 0;

    let c: Vec<char> = s.chars().collect();
    let mut index_map = HashMap::new();
    while right < s.len() {
        // 若right指针处的字符已经出现在当前窗口内
        if index_map.contains_key(&c[right]) {
            // 则计算当前长度
            max_len = max_len.max(right - left);
            // 让left跳至重复字符后, 同时从map中移除此前的字符
            let repeat_index = *index_map.get(&c[right]).unwrap();
            for i in left..=repeat_index {
                index_map.remove(&c[i]);
            }
            left = repeat_index + 1;
        }
        index_map.insert(c[right], right);
        // 由于重复时，left会移动，所以此时right处字符肯定不重复，继续往后找
        right += 1;
    }

    max_len = max_len.max(right - left);
    max_len as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn length_of_longest_substring_test() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("abba".to_string()), 2);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
