// 204. 计数质数
// 给定整数 n ，返回 所有小于非负整数 n 的质数的数量 。

// 示例 1：

// 输入：n = 10
// 输出：4
// 解释：小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
// 示例 2：

// 输入：n = 0
// 输出：0
// 示例 3：

// 输入：n = 1
// 输出：0

// 提示：

// 0 <= n <= 5 * 106

use std::time::Duration;

fn prime_count(num: usize) -> usize {
    let mut ans = 0;
    for i in 1..num {
        if is_prime(i) {
            ans += 1;
        }
    }
    ans
}

fn eratosthenes_sieve(num: usize) -> usize {
    if num <= 1 {
        return 0;
    }

    let mut is_prime = vec![true; num];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..num {
        if !is_prime[i] {
            continue;
        }
        for j in (2 * i..num).step_by(i) {
            is_prime[j] = false;
        }
    }

    is_prime.iter().filter(|&&b| b).count()
}

fn linear_sieve(num: usize) -> usize {
    if num <= 1 {
        return 0;
    }

    let mut is_prime = vec![true; num];
    (is_prime[0], is_prime[1]) = (false, false);

    let mut prime = Vec::new();
    for i in 2..num {
        if is_prime[i] {
            prime.push(i);
        }
        for &p in prime.iter().take_while(|&&p| i * p < num) {
            is_prime[i * p] = false;
            if i % p == 0 {
                break;
            }
        }
    }

    is_prime.iter().filter(|&&b| b).count()
}

fn is_prime(num: usize) -> bool {
    if num <= 1 {
        return false;
    }

    let sqrt = (num as f64).sqrt();
    let sqrt = sqrt as u32;
    for i in 2..=sqrt as usize {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn is_prime_test() {
        let prime_list = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for i in prime_list.iter() {
            assert!(is_prime(*i));
        }

        let composite_list = vec![4, 6, 8, 9, 10, 12, 14, 15, 16];
        for i in composite_list.iter() {
            assert!(!is_prime(*i));
        }
    }

    #[test]
    fn prime_count_test() {
        assert_eq!(prime_count(10), 4);
        assert_eq!(prime_count(0), 0);
        assert_eq!(prime_count(1), 0);
        assert_eq!(prime_count(2), 0);
        assert_eq!(prime_count(461465), 38571);
    }

    #[test]
    fn eratosthenes_sieve_test() {
        assert_eq!(eratosthenes_sieve(10), 4);
        assert_eq!(eratosthenes_sieve(0), 0);
        assert_eq!(eratosthenes_sieve(1), 0);
        assert_eq!(eratosthenes_sieve(2), 0);
        assert_eq!(eratosthenes_sieve(461465), 38571);
    }

    #[test]
    fn linear_sieve_test() {
        assert_eq!(linear_sieve(10), 4);
        assert_eq!(linear_sieve(0), 0);
        assert_eq!(linear_sieve(1), 0);
        assert_eq!(linear_sieve(2), 0);
        assert_eq!(linear_sieve(461465), 38571);
    }

    #[test]
    fn duration_test() {
        let num = 5000000;

        let start = Instant::now();
        println!("{}", prime_count(num));
        println!("count duration: {:?}", start.elapsed());

        let start = Instant::now();
        println!("{}", eratosthenes_sieve(num));
        println!("eratosthenes sieve duration: {:?}", start.elapsed());

        let start = Instant::now();
        println!("{}", linear_sieve(num));
        println!("linear sieve duration: {:?}", start.elapsed());
    }
}
