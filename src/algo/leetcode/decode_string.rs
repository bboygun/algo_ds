// 394. 字符串解码
// 给定一个经过编码的字符串，返回它解码后的字符串。
// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。

// 示例 1：
// 输入：s = "3[a]2[bc]"
// 输出："aaabcbc"

// 示例 2：
// 输入：s = "3[a2[c]]"
// 输出："accaccacc"

// 示例 3：
// 输入：s = "2[abc]3[cd]ef"
// 输出："abcabccdcdcdef"

// 示例 4：
// 输入：s = "abc3[cd]xyz"
// 输出："abccdcdcdxyz"

// 提示：
// 1 <= s.length <= 30
// s 由小写英文字母、数字和方括号 '[]' 组成
// s 保证是一个 有效 的输入。
// s 中所有整数的取值范围为 [1, 300]

fn decode_string(s: String) -> String {
    let mut stack = vec![];

    for c in s.chars().into_iter() {
        if c != ']' {
            stack.push(c);
            continue;
        }

        let mut sub_string = String::new();
        while let Some(last) = stack.pop() {
            if last.is_lowercase() {
                sub_string.push(last);
            } else {
                break;
            }
        }

        let mut time_string = String::new();
        while let Some(last) = stack.pop() {
            if last.is_numeric() {
                time_string.push(last);
            } else {
                stack.push(last);
                break;
            }
        }
        time_string = time_string.chars().rev().collect();

        let mut time = time_string.parse::<usize>().unwrap();
        sub_string = sub_string.repeat(time);
        stack.extend(sub_string.chars().rev());
    }

    stack.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decode_string_test() {
        let case1 = "3[a]2[bc]";
        let ans1 = "aaabcbc";
        assert_eq!(decode_string(case1.into()), ans1.to_string());

        let case2 = "3[a2[c]]";
        let ans2 = "accaccacc";
        assert_eq!(decode_string(case2.into()), ans2.to_string());

        let case3 = "2[abc]3[cd]ef";
        let ans3 = "abcabccdcdcdef";
        assert_eq!(decode_string(case3.into()), ans3.to_string());

        let case4 = "abc3[cd]xyz";
        let ans4 = "abccdcdcdxyz";
        assert_eq!(decode_string(case4.into()), ans4.to_string());
    }
}
