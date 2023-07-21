pub struct Solution;

impl Solution {
    #[inline]
    pub fn count_servers(
        n: i32,
        logs: Vec<Vec<i32>>,
        x: i32,
        queries: Vec<i32>,
    ) -> Vec<i32> {
        let mut max = 0;
        let mut min = i32::MAX;
        for log in &logs {
            max = std::cmp::max(log[1] + x + 1, max);
            min = std::cmp::min(log[1], min);
        }
        let len = max - min;
        let mut start = vec![vec![]; len as usize];
        let mut end = vec![vec![]; len as usize];
        for log in &logs {
            start[(log[1] - min) as usize].push(log[0]);
            end[(log[1] + x - min) as usize].push(log[0]);
        }

        let mut available = vec![0; (n + 1) as usize];
        let mut mp = vec![0; len as usize];
        let mut count = 0;
        for i in min..max {
            for id in &start[(i - min) as usize] {
                if available[*id as usize] == 0 {
                    count += 1;
                }
                available[*id as usize] += 1;
            }
            mp[(i - min) as usize] = count;
            for id in &end[(i - min) as usize] {
                available[*id as usize] -= 1;
                if available[*id as usize] == 0 {
                    count -= 1;
                }
            }
        }
        let mut res = vec![n; queries.len()];
        for i in 0..queries.len() {
            let query = queries[i];
            if query >= min && query < max {
                res[i] -= mp[(query - min) as usize];
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test1() {
        let res = Solution::count_servers(
            3,
            vec![vec![1, 3], vec![2, 6], vec![1, 5]],
            5,
            vec![10, 11],
        );
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let res = Solution::count_servers(
            3,
            vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]],
            2,
            vec![3, 4],
        );
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn test3() {
        let res = Solution::count_servers(
            1,
            vec![vec![1, 628080]],
            70756,
            vec![468829],
        );
        assert_eq!(res, vec![1]);
    }
}
