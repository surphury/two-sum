mod test;

use std::convert::TryInto;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indexes: Vec<i32> = Vec::new();

    'outter: for index in 0..nums.len() {
        let current = nums[index];

        for i in 0..nums.len() {
            if i == index {
                continue;
            } else {
                if nums[i] + current == target {
                    indexes.push(index.try_into().unwrap());
                    indexes.push(i.try_into().unwrap());
                    break 'outter;
                }
            }
        }
    }

    indexes
}

fn main() {}
