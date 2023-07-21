pub struct Solution;

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut rows = vec![-1; n];
        let mut cols = vec![-1; n];
        let len = queries.len();
        let mut sorted_queries = Vec::with_capacity(2 * n);
        for i in 0..len {
            let query = &queries[i];
            let is_row = query[0] == 0;
            let pos = query[1] as usize;
            let val = query[2];
            if is_row {
                if rows[pos] >= 0 {
                    sorted_queries[rows[pos] as usize] = 0;
                }
                rows[pos] = sorted_queries.len() as i32;
                sorted_queries.push(val + 1);
            } else {
                if cols[pos] >= 0 {
                    sorted_queries[cols[pos] as usize] = 0;
                }
                cols[pos] = sorted_queries.len() as i32;
                sorted_queries.push(-val - 1);
            }
        }
        let mut res = 0;
        let mut row_sum = 0;
        let mut col_sum = 0;
        let len = sorted_queries.len();
        let n = n as i64;
        for i in 0..len {
            let val = sorted_queries[i] as i64;
            if val == 0 {
                continue;
            } else if val > 0 {
                res += (val - 1) * n - col_sum;
                row_sum += val - 1;
            } else {
                res += (-val - 1) * n - row_sum;
                col_sum += -val - 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::matrix_sum_queries(
            3,
            vec![vec![0, 0, 1], vec![1, 2, 2], vec![0, 2, 3], vec![1, 0, 4]],
        );
        assert_eq!(res, 23);
    }

    #[test]
    fn test2() {
        let res = Solution::matrix_sum_queries(
            3,
            vec![
                vec![0, 0, 4],
                vec![0, 1, 2],
                vec![1, 0, 1],
                vec![0, 2, 3],
                vec![1, 2, 1],
            ],
        );
        assert_eq!(res, 17);
    }
}
