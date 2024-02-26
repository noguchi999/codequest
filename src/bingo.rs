use rand::seq::SliceRandom;

pub fn main() {
    let mut nums = [0; 75];
    for i in 1..=75 {
        nums[i - 1] = i;
    }
}