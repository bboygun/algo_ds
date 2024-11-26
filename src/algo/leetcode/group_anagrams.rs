// 49. 字母异位词分组
// 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
// 字母异位词 是由重新排列源单词的所有字母得到的一个新单词。

// 示例 1:
// 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
// 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]

// 示例 2:
// 输入: strs = [""]
// 输出: [[""]]

// 示例 3:
// 输入: strs = ["a"]
// 输出: [["a"]]

// 提示：
// 1 <= strs.length <= 104
// 0 <= strs[i].length <= 100
// strs[i] 仅包含小写字母

use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<&String, Vec<&String>> = HashMap::new();
    map.insert(&strs[0], vec![&strs[0]]);

    for s in strs.iter().skip(1) {
        let mut new_key = true;
        for (key, value) in map.iter_mut() {
            if is_anagrams(key, &s) {
                value.push(&s);
                new_key = false;
                break;
            }
        }
        if new_key {
            map.insert(&s, vec![&s]);
        }
    }

    let mut ans: Vec<Vec<String>> = vec![];
    for list in map.values() {
        let mut cur_list: Vec<String> = vec![];
        for i in 0..list.len() {
            cur_list.push(list[i].clone());
        }
        ans.push(cur_list);
    }
    ans
}

fn is_anagrams(word1: &str, word2: &str) -> bool {
    let (mut map1, mut map2) = (HashMap::new(), HashMap::new());
    for c in word1.chars() {
        *map1.entry(c).or_insert(0) += 1;
    }
    for c in word2.chars() {
        *map2.entry(c).or_insert(0) += 1;
    }
    map1 == map2
}

fn group_anagrams_sort(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for str in strs {
        let mut sorted_char: Vec<char> = str.chars().collect();
        sorted_char.sort();
        let key = sorted_char.into_iter().collect::<String>();

        if let Some(list) = map.get_mut(&key) {
            list.push(str);
        } else {
            map.insert(key.clone(), vec![str]);
        }
    }

    let mut ans: Vec<Vec<String>> = Vec::new();
    for list in map.values() {
        ans.push(list.clone());
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn group_anagrams_test() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_vec_vec_eq(group_anagrams(strs.clone()), result.clone());
        assert_vec_vec_eq(group_anagrams_sort(strs), result.clone());
    }

    fn assert_vec_vec_eq(mut vec1: Vec<Vec<String>>, mut vec2: Vec<Vec<String>>) {
        for v1 in &mut vec1 {
            v1.sort();
        }
        vec1.sort();

        for v2 in &mut vec2 {
            v2.sort();
        }
        vec2.sort();

        assert_eq!(vec1, vec2);
    }
}
