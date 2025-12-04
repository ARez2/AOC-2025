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
        let orig_dist = line[1..].parse::<i32>().unwrap();
        // count full rotations
        let num_full_rotations = orig_dist / 100;
        // now discard the full rotation
        let dist = orig_dist % 100;
        let next_not_wrapped = current + dir * dist;
        let next_wrapped = next_not_wrapped.rem_euclid(100);

        let mut num_zeros = num_full_rotations;
        // check for wrapping by comparing unwrapped vs. wrapped number
        match dir {
            1 => {
                if next_wrapped < next_not_wrapped {
                    num_zeros += 1;
                }
            }
            -1 => {
                // when moving left we can also end up at 0 like 6 + (-6) = 0
                if current != 0 && (next_wrapped > next_not_wrapped || next_wrapped == 0) {
                    num_zeros += 1;
                }
            }
            _ => (),
        }

        solution += num_zeros;

        // println!(
        //     "{:>22}    Full rots: {}, Extra zeroes: {}  Solution: {}",
        //     format!(
        //         "{} + {} -> {} ({})",
        //         current,
        //         dir * orig_dist,
        //         next_not_wrapped,
        //         next_wrapped
        //     ),
        //     num_full_rotations,
        //     num_zeros - num_full_rotations,
        //     solution
        // );
        current = next_wrapped;
    }
    Some(solution as u64)
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

        let start = 50;
        let mut current = start;
        let mut solution = 0;
        let line = "L150";
        let dir = (line.starts_with("R")) as i32 - (line.starts_with("L")) as i32;
        let delta = dir * line[1..].parse::<i32>().unwrap();
        let next_not_wrapped = current + delta;
        let next_wrapped = next_not_wrapped.rem_euclid(100);
        let num_wraps = ((next_not_wrapped - next_wrapped) / 100).abs();
        solution += num_wraps;
        if next_wrapped == 0 {
            solution += 1;
        }
        println!(
            "{} -> {} ({})    Wraps: {}",
            current, next_not_wrapped, next_wrapped, num_wraps
        );
        current = next_wrapped;
    }
}
