use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    match two_sum(&nums, target) {
        Some((i, j)) => println!("Indices: ({}, {}) → {} + {} = {}", i, j, nums[i], nums[j], target),
        None => println!("No solution found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn test_middle() {
        assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    }

    #[test]
    fn test_same_element() {
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn test_no_solution() {
        assert_eq!(two_sum(&[1, 2, 3], 10), None);
    }

    #[test]
    fn test_negatives() {
        assert_eq!(two_sum(&[-1, -2, -3, -4, -5], -8), Some((2, 4)));
    }
}
