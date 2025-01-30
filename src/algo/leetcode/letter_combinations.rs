// 17. 电话号码的字母组合
// 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

// 示例 1：
// 输入：digits = "23"
// 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
//
// 示例 2：
// 输入：digits = ""
// 输出：[]

// 示例 3：
// 输入：digits = "2"
// 输出：["a","b","c"]

// 提示：
// 0 <= digits.length <= 4
// digits[i] 是范围 ['2', '9'] 的一个数字。

fn get_letters(digit: &u8) -> Vec<&'static str> {
    match digit {
        2 => vec!["a", "b", "c"],
        3 => vec!["d", "e", "f"],
        4 => vec!["g", "h", "i"],
        5 => vec!["j", "k", "l"],
        6 => vec!["m", "n", "o"],
        7 => vec!["p", "q", "r", "s"],
        8 => vec!["t", "u", "v"],
        9 => vec!["w", "x", "y", "z"],
        _ => panic!(),
    }
}

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }

    let mut result = vec![];
    let mut track = vec![];

    let digits: Vec<u8> = digits.chars().map(|digit| (digit as u8) - b'0').collect();
    backtrace(0, &digits, &mut result, &mut track);

    result
}

fn backtrace(start: usize, digits: &[u8], result: &mut Vec<String>, track: &mut Vec<&str>) {
    if digits.len() == track.len() {
        result.push(track.join(""));
        return;
    }

    let digit = digits[start];
    let alpha_ref = get_letters(&digit);
    for &ch in alpha_ref.iter() {
        track.push(ch);
        backtrace(start + 1, digits, result, track);
        track.pop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn letter_combinations_test() {
        let case1 = "23".to_string();
        let ans1 = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(letter_combinations(case1), ans1);
    }
}
