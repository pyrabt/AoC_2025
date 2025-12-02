advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut num_zeros: u64 = 0;
    let mut current: i32 = 50;
    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        if dir == "L" {
            current = (current - num.parse::<i32>().unwrap()).rem_euclid(100);
        } else {
            current = (current + num.parse::<i32>().unwrap()).rem_euclid(100);
        }

        if current == 0 {
            num_zeros += 1;
        }
    }
    Some(num_zeros)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut num_zeros: i32 = 0;
    let mut current: i32 = 50;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let parsed_num: i32 = num.parse::<i32>().unwrap();
        let mut distance_from_zero: i32 = current;

        if dir == "L" {
            if current <= 0 {
                distance_from_zero = 100 + current;
            }
            if distance_from_zero <= parsed_num {
                num_zeros += 1;
                num_zeros += (parsed_num - distance_from_zero) / 100;
            }
            current = (current - parsed_num).rem_euclid(100);
        } else {
            distance_from_zero = 100 - current;
            if distance_from_zero <= parsed_num {
                num_zeros += 1;
                num_zeros += (parsed_num - distance_from_zero) / 100;
            }
            current = (current + parsed_num).rem_euclid(100);
        }
    }
    Some(num_zeros)
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
