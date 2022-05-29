use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                    break
                }
            }
        }
        return result;
    }
}

struct Solution2 {}

impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        let mut map = HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            map.insert(nums[i], i as i32);
        }

        for i in 0..nums.len() {
            let rest = target - nums[i];
            if map.contains_key(&rest) && i != map[&rest] as usize {
                return vec![i as i32, map[&rest]];
            }
        }
        return result;
    }
}

struct Solution3 {}

impl Solution3 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        let mut map = HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            let rest = target - nums[i];
            if map.contains_key(&rest) {
                return vec![map[&rest], i as i32];
            }
            map.insert(nums[i], i as i32);
        }

        return result;
    }
}

fn main() {
    let input = vec![2, 5, 5, 11];
    let target = 10;
    // let sol = Solution::two_sum(input, target);
    // let sol = Solution2::two_sum(input, target);
    let sol = Solution3::two_sum(input, target);
    println!("{}, {}", sol[0], sol[1]);
}

