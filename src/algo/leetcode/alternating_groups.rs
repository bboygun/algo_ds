fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let len = colors.len();
    let mut ans = 0;
    for (index, &color) in colors.iter().enumerate() {
        let left = get_left(&colors, index, len);
        let right = get_right(&colors, index, len);
        if left != color && color != right {
            ans += 1;
        }
    }
    ans
}

fn get_left(colors: &Vec<i32>, index: usize, len: usize) -> i32 {
    let left_index = if index == 0 { len - 1 } else { index - 1 };
    colors[left_index]
}

fn get_right(colors: &Vec<i32>, index: usize, len: usize) -> i32 {
    let right_index = if index == len - 1 { 0 } else { index + 1 };
    colors[right_index]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn alternating_groups_test() {
        let colors = vec![1, 1, 1];
        assert_eq!(number_of_alternating_groups(colors), 0);

        let colors = vec![0, 1, 0, 0, 1];
        assert_eq!(number_of_alternating_groups(colors), 3);
    }
}
