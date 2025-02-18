// 20. 有效的括号
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
// 有效字符串需满足：
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
// 每个右括号都有一个对应的相同类型的左括号。

// 示例 1：
// 输入：s = "()"
// 输出：true

// 示例 2：
// 输入：s = "()[]{}"
// 输出：true

// 示例 3：
// 输入：s = "(]"
// 输出：false

// 示例 4：
// 输入：s = "([])"
// 输出：true

// 提示：
// 1 <= s.length <= 10^4
// s 仅由括号 '()[]{}' 组成

use std::collections::{HashMap, HashSet};

fn is_valid(s: String) -> bool {
    let left_parentheses: HashSet<char> = ['(', '[', '{'].into_iter().collect();
    let left_map: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')].into_iter().collect();
    let mut stack: Vec<char> = vec![];
    for c in s.chars().into_iter() {
        if left_parentheses.contains(&c) {
            stack.push(c);
            continue;
        }
        let last = stack.pop();
        if last.is_none() {
            return false;
        }
        let last = last.unwrap();
        if &last != left_map.get(&c).unwrap() {
            return false;
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_parentheses_test() {
        let case1 = "()";
        let case2 = "()[]{}";
        let case3 = "(]";
        let case4 = "([])";

        assert_eq!(is_valid(case1.into()), true);
        assert_eq!(is_valid(case2.into()), true);
        assert_eq!(is_valid(case3.into()), false);
        assert_eq!(is_valid(case4.into()), true);
    }
}
