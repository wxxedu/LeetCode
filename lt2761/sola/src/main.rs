pub struct Solution;

impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        if n < 3 {
            return vec![];
        }
        let n = n as usize;

        let mut primes: Vec<bool> = vec![true; n];
        let max_divisor = (n as f64).sqrt().ceil() as usize;
        for i in 2..max_divisor + 1 {
            let mut idx = i + i;
            while idx < n {
                primes[idx] = false;
                idx += i;
            }
        }

        primes[0] = false;
        primes[1] = false;

        let mut res = vec![];

        for i in 2..n / 2 + 1 {
            let other = n - i;
            if primes[i] && primes[other] {
                res.push(vec![i as i32, other as i32]);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let res = Solution::find_prime_pairs(10);
        assert_eq!(res, vec![vec![3, 7], vec![5, 5]]);
    }

    #[test]
    fn test_2() {
        let res = Solution::find_prime_pairs(2);
        assert_eq!(res, vec![vec![0; 0]; 0]);
    }
}

fn main() {
    Solution::find_prime_pairs(10);
}
