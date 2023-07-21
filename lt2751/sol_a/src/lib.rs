use std::fmt::Display;

pub struct Solution;

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Robot_{}(pos: {}, dir: {}, health: {})",
            self.order,
            self.position,
            if self.is_right { "->" } else { "<-" },
            self.health
        )
    }
}

fn print_robots(robots: &[Robot]) {
    println!("[");
    for robot in robots {
        println!("\t{},", robot);
    }
    println!("]");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Robot {
    pub position: i32,
    pub is_right: bool,
    pub order: usize,
    pub health: i32,
}

impl Robot {
    #[inline(always)]
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    #[inline(always)]
    pub fn fight(&mut self, other: &mut Robot) {
        if self.health > other.health {
            self.health -= 1;
            other.health = 0;
        } else if self.health == other.health {
            self.health = 0;
            other.health = 0;
        } else {
            other.health -= 1;
            self.health = 0;
        }
    }
}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let mut robots = Vec::with_capacity(n);
        let mut directions = directions.chars();
        for i in 0..n {
            robots.push(Robot {
                position: positions[i],
                is_right: directions.nth(0).unwrap() == 'R',
                order: i,
                health: healths[i],
            });
        }
        robots.sort();
        let mut alive_right_robots =
            std::collections::VecDeque::with_capacity(n);
        let mut left_robots = Vec::with_capacity(n);
        for i in 0..n {
            let mut robot = robots[i].clone();
            if robot.is_right {
                alive_right_robots.push_back(robot);
                continue;
            }
            while !alive_right_robots.is_empty() && robot.is_alive() {
                let mut last_robot = alive_right_robots.pop_back().unwrap();
                robot.fight(&mut last_robot);
                if last_robot.is_alive() {
                    alive_right_robots.push_back(last_robot);
                }
            }
            if robot.is_alive() {
                left_robots.push(robot);
            }
        }
        for robot in alive_right_robots {
            left_robots.push(robot);
        }
        left_robots.sort_by(|x, y| x.order.cmp(&y.order));
        left_robots.iter().map(|x| x.health).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::survived_robots_healths(
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR".to_owned(),
        );
        assert_eq!(res, vec![2, 17, 9, 15, 10]);
    }

    #[test]
    fn test2() {
        let res = Solution::survived_robots_healths(
            vec![3, 5, 2, 6],
            vec![10, 10, 15, 12],
            "RLRL".to_owned(),
        );
        assert_eq!(res, vec![14]);
    }

    #[test]
    fn test3() {
        let res = Solution::survived_robots_healths(
            vec![1, 2, 5, 6],
            vec![10, 10, 11, 11],
            "RLRL".to_owned(),
        );
        assert_eq!(res, vec![]);
    }
}
