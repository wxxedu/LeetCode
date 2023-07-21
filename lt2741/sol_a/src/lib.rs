pub struct Solution;

const MODULO: i32 = 1000_000_007;

impl Solution {
    #[inline]
    fn visit(
        node: usize,
        visited: i32,
        len: usize,
        level: usize,
        graph: &Vec<Vec<bool>>,
        cache: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if len == level {
            return 1;
        }
        if cache[node][level][visited as usize] >= 0 {
            println!(
                "-> cache[node][{}][{:03b}] = {}",
                level, visited, cache[node][level][visited as usize]
            );
            return cache[node][level][visited as usize];
        }
        let mut res = 0;
        for next in 0..len {
            let next_mask = 0b1 << next;
            if (next_mask & visited) <= 0 {
                continue;
            }
            if graph[node as usize][next as usize] {
                res += Self::visit(
                    next,
                    visited ^ next_mask,
                    len,
                    level + 1,
                    graph,
                    cache,
                );
                res = res % MODULO;
            }
        }
        println!("cache[node][{}][{:2b}] = {}", level, visited, res);
        cache[node][level][visited as usize] = res;
        res
    }

    #[inline]
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut graph = vec![vec![false; nums.len()]; nums.len()];
        for i in 0..nums.len() - 1 {
            let num = nums[i];
            for j in i + 1..nums.len() {
                let bigger_num = nums[j];
                if bigger_num % num == 0 {
                    graph[i][j] = true;
                    graph[j][i] = true;
                }
            }
        }
        let mut res = 0;
        let visited = (1 << nums.len()) - 1;
        let mut dp =
            vec![vec![vec![-1; 1 << nums.len()]; nums.len()]; nums.len()];
        for node in 0..nums.len() {
            let next_mask = 0b1 << node;
            res += Self::visit(
                node,
                visited ^ next_mask,
                nums.len(),
                1,
                &graph,
                &mut dp,
            );
            res = res % MODULO;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 6, 6, _
        let res = Solution::special_perm(vec![2, 3, 6]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test2() {
        // (4, 3), _, _
        let res = Solution::special_perm(vec![1, 4, 3]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test3() {
        // (6, 8), (6), _, _
        let res = Solution::special_perm(vec![2, 3, 6, 8]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test4() {
        let res = Solution::special_perm(vec![
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
        ]);
        assert_eq!(res, 178290591);
    }
}
