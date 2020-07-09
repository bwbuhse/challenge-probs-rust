/// https://leetcode.com/problems/3sum/
use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut sols = Vec::new();
    let mut checked = HashSet::new();

    for i in 0..nums.len() {
        if !checked.contains(&nums[i]) {
            checked.insert(nums[i]);
            let new_target = target - nums[i];
            let mut new_nums = nums.clone();

            new_nums.remove(i);

            let two_sum_sols = two_sum(new_nums, new_target);
            for pair in two_sum_sols {
                let mut triple = pair.clone();
                triple.push(nums[i]);
                sols.push(triple);
            }
        }
    }

    sols
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut sols = Vec::new();
    let mut checked = HashSet::new();

    for i in 0..nums.len() {
        if !checked.contains(&nums[i]) {
            checked.insert(nums[i]);
            let mut inner_checked = HashSet::new();

            for j in (i + 1)..nums.len() {
                if !inner_checked.contains(&nums[j]) {
                    inner_checked.insert(nums[j]);
                    if target - nums[i] == nums[j] {
                        // println!("{} - {} ?= {}", target, i as i32, j as i32);
                        sols.push(vec![nums[i], nums[j]]);
                    }
                }
            }
        }
    }

    sols
}
