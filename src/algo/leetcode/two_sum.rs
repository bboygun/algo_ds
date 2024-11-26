use std::collections::HashMap;

fn two_sum(num: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index, &value) in num.iter().enumerate() {
        let to_search = target - num[index];
        if map.contains_key(&to_search) {
            return vec![*map.get(&to_search).unwrap() as i32, index as i32];
        }
        map.insert(value, index);
    }

    vec![]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn two_sum_test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);

        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);

        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
