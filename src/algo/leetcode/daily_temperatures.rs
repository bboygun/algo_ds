// 739. 每日温度
// 给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i 天，下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。

// 示例 1:
// 输入: temperatures = [73,74,75,71,69,72,76,73]
// 输出: [1,1,4,2,1,1,0,0]

// 示例 2:
// 输入: temperatures = [30,40,50,60]
// 输出: [1,1,1,0]

// 示例 3:
// 输入: temperatures = [30,60,90]
// 输出: [1,1,0]

// 提示：
// 1 <= temperatures.length <= 10^5
// 30 <= temperatures[i] <= 100

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack = vec![];

    for (index, &value) in temperatures.iter().enumerate() {
        while let Some(last_index) = stack.pop() {
            if value > temperatures[last_index] {
                result[last_index] = (index - last_index) as i32;
            } else {
                stack.push(last_index);
                break;
            }
        }
        stack.push(index);
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn daily_temperatures_test() {
        let case1 = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let ans1 = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(daily_temperatures(case1), ans1);

        let case2 = vec![30, 40, 50, 60];
        let ans2 = vec![1, 1, 1, 0];
        assert_eq!(daily_temperatures(case2), ans2);

        let case3 = vec![30, 60, 90];
        let ans3 = vec![1, 1, 0];
        assert_eq!(daily_temperatures(case3), ans3);
    }
}
