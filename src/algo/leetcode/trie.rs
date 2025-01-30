// 208. 实现 Trie (前缀树)
// Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补全和拼写检查。

// 请你实现 Trie 类：
// Trie() 初始化前缀树对象。
// void insert(String word) 向前缀树中插入字符串 word 。
// boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回 false 。
// boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true ；否则，返回 false 。

// 示例：

// 输入
// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
// 输出
// [null, null, true, false, true, null, true]

// 解释
// Trie trie = new Trie();
// trie.insert("apple");
// trie.search("apple");   // 返回 True
// trie.search("app");     // 返回 False
// trie.startsWith("app"); // 返回 True
// trie.insert("app");
// trie.search("app");     // 返回 True

// 提示：
// 1 <= word.length, prefix.length <= 2000
// word 和 prefix 仅由小写英文字母组成
// insert、search 和 startsWith 调用次数 总计 不超过 3 * 104 次

use std::{cell::RefCell, collections::HashMap};

struct Trie {
    is_end: RefCell<bool>,
    children: RefCell<HashMap<char, Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_end: RefCell::new(false),
            children: RefCell::new(HashMap::new()),
        }
    }

    fn insert(&self, word: String) {
        let mut current = self;
        for c in word.chars().into_iter() {
            let next = {
                let mut children = current.children.borrow_mut();
                children.entry(c).or_insert(Self::new()) as *const Trie
            };
            current = unsafe { &*next };
        }
        *current.is_end.borrow_mut() = true;
    }

    fn search(&self, word: String) -> bool {
        match self.find_with_prefix(word) {
            Some(result) => *result.is_end.borrow(),
            None => false,
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_with_prefix(prefix).is_some()
    }

    fn find_with_prefix(&self, prefix: String) -> Option<&Trie> {
        let mut current = self;
        for c in prefix.chars() {
            let children = current.children.borrow();
            if let Some(child) = children.get(&c) {
                // 此处安全，因为child为Trie，且其生命周期与整个前缀树相同
                current = unsafe { &*(child as *const Trie) };
            } else {
                return None;
            }
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trie_test() {
        let trie = Trie::new();
        trie.insert(String::from("apple"));
        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), false);
        assert_eq!(trie.starts_with(String::from("app")), true);
        trie.insert(String::from("app"));
        assert_eq!(trie.search(String::from("app")), true);
    }
}
