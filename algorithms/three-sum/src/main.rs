use std::collections::HashMap;

struct Solution {}

impl Solution {
    // seek next un-repeat number
    #[inline(always)]
    fn next_unique(nums: &[i32], idx: usize, forward: bool) -> usize {
        let curr = nums[idx];
        let mut i = idx;
        while i > 0 && i < nums.len() && nums[i] == curr {
            i = if forward { i + 1 } else { i - 1 }
        }
        i
    }
    #[inline(always)]
    fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        let mut result = Vec::new();
        while i < j {
            if nums[i] + nums[j] < sum { i += 1 }
            else if nums[i] + nums[j] > sum { j -= 1 }
            else {
                result.push((nums[i], nums[j]));
                i = Solution::next_unique(nums, i, true);
                j = Solution::next_unique(nums, j, false);
            }
        }
        result
    }
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        let mut previous = nums[0] - 1;

        let mut result = Vec::<Vec::<i32>>::new();

        for i in 0..len -2 {
            if nums[i] == previous { continue }
            previous = nums[i];
            let n = nums[i];
            let rest = 0 - n;
            let mut ret = Solution::two_sum(&nums[(i+1)..len], rest);
            for t in ret.iter() {
                result.push(vec![nums[i], t.0, t.1]);
            }
        }
        return result;
    }
}

fn main() {
    let input = vec![1];
    let sol = Solution::three_sum(input);
    println!("{:?}", sol)
}
