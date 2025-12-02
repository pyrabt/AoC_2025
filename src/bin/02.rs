use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for num in start..=end {
            let num_string = num.to_string();
            if num_string.len() % 2 != 0 {
                continue;
            }
            let (left, right) = num_string.split_at(num_string.len() / 2);
            if left == right {
                invalid_ids.push(num);
            }
        }
    }
    Some(invalid_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for num in start..=end {
            let num_string = num.to_string();
            for c in 2..=num_string.len() {
                if num_string.len() % c != 0 {
                    continue;
                }
                let id_chunks: Vec<String> = get_chunks(num_string.clone(), num_string.len()/c);
                if id_chunks.iter().all_equal() {
                    invalid_ids.push(num);
                    break;
                }
            }
        }
    }
    Some(invalid_ids.iter().sum())
}

fn get_chunks(input: String, chunk_size: usize) -> Vec<String> {
    input
        .chars()
        .chunks(chunk_size)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
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
