use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges_strs = input.split(",");
    let ranges: Vec<RangeInclusive<u64>> = ranges_strs
        .clone()
        .map(|range_str| {
            let ends = range_str.trim().split("-").collect::<Vec<&str>>();
            let start = ends[0].parse::<u64>().unwrap();
            let end = ends[1].parse::<u64>().unwrap();
            start..=end
        })
        .collect();

    let mut solution = 0;
    for range in ranges {
        for id in range {
            let id_str = id.to_string();
            let total_chars = id_str.len();

            // Since false ids always have repetition in them,
            // they need to be of even length
            if total_chars % 2 != 0 {
                continue;
            }
            let split = id_str.split_at(total_chars / 2);
            let seq_1 = split.0;
            let seq_2 = split.1;

            if seq_1 == seq_2 {
                #[cfg(test)]
                println!("{}", id);
                solution += id;
            }
        }
    }
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges_strs = input.split(",");
    let ranges: Vec<RangeInclusive<u64>> = ranges_strs
        .clone()
        .map(|range_str| {
            let ends = range_str.trim().split("-").collect::<Vec<&str>>();
            let start = ends[0].parse::<u64>().unwrap();
            let end = ends[1].parse::<u64>().unwrap();
            start..=end
        })
        .collect();

    let mut solution = 0;
    for range in ranges {
        for id in range {
            let id_str = id.to_string();
            let total_chars = id_str.len();

            'splitting_loop: for part_size in 1..total_chars {
                // check if we can divide the number into part_size-equal parts
                if total_chars % part_size != 0 {
                    continue;
                }
                let num_parts = total_chars / part_size;
                let mut prev_part = &id_str[0..part_size];
                #[cfg(test)]
                println!("{} - size: {} -> num parts: {}", id, part_size, num_parts);
                for part_nr in 1..num_parts {
                    let prev_part_end = part_nr * part_size;
                    let this_part_end = prev_part_end + part_size;
                    let this_part = &id_str[prev_part_end..this_part_end];
                    #[cfg(test)]
                    println!(
                        "{} =?= {} : {}",
                        prev_part,
                        this_part,
                        this_part == prev_part
                    );
                    if this_part != prev_part {
                        #[cfg(test)]
                        println!();
                        continue 'splitting_loop;
                    }
                    prev_part = this_part;
                }
                // println!("{}", id);
                // if we reached this point it means that all parts were the same
                // meaning we got a repeating pattern
                // meaning the id is invalid
                solution += id;
                #[cfg(test)]
                println!("ID INVALID");
                #[cfg(test)]
                println!();
                break 'splitting_loop;
            }
        }
    }

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
