use num_traits::Num;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let start = 50;
    let mut current = start;
    let mut solution = 0;
    for line in input.trim().split("\n") {
        let dir = (line.starts_with("R")) as i32 - (line.starts_with("L")) as i32;
        let delta = dir * line[1..].parse::<i32>().unwrap();
        current = (current + delta) % 100;
        if current == 0 {
            solution += 1;
        }
    }
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start = 50;
    let mut current = start;
    let mut solution = 0;
    for line in input.trim().split("\n") {
        let dir = (line.starts_with("R")) as i32 - (line.starts_with("L")) as i32;
        let delta = dir * line[1..].parse::<i32>().unwrap();
        let next_not_wrapped = current + delta;
        let next_wrapped = next_not_wrapped.rem_euclid(100);
        let mut went_over_zero = match dir {
            1 => {
                // if we go right, we need to watch out for when we cross 100 (and would wrap over zero)
                // or if we land directly on zero
                current.signum() == next_wrapped.signum() || next_wrapped.signum() == 0
            }
            -1 => current.signum() != next_not_wrapped.signum() || next_wrapped.signum() == 0,
            _ => false,
        };
        if went_over_zero {
            solution += 1;
        }
        current = next_wrapped;
    }
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
