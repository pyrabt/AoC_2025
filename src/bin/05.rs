advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut num_fresh: u64 = 0;

    let mut range_section_complete: bool = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            range_section_complete = true;
            continue;
        }
        if !range_section_complete {
            let range: Vec<usize> = line
                .trim()
                .split("-")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            // naive
            ranges.push((range[0], range[1]));
        } else {
            let id = line.trim().parse::<usize>().unwrap();
            for r in 0..ranges.len() {
                if id >= ranges[r].0 && id <= ranges[r].1 {
                    num_fresh += 1;
                    break;
                }
            }
        }
    }
    Some(num_fresh)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }
        let current_range: Vec<usize> = line
            .trim()
            .split("-")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        ranges.push((current_range[0], current_range[1]));
    }
    let mut modified: bool = true;
    while modified {
        modified = false;
        for c in 0..ranges.len() {
            for r in 0..ranges.len() {
                if r == c {
                    continue;
                }
                // lower overlap
                if ranges[c].0 < ranges[r].0
                    && ranges[c].1 <= ranges[r].1
                    && ranges[c].1 > ranges[r].0
                {
                    ranges[r].0 = ranges[c].0;
                    ranges.remove(c);
                    modified = true;
                    break;
                }
                // upper overlap
                if ranges[c].0 <= ranges[r].1
                    && ranges[c].0 > ranges[r].0
                    && ranges[c].1 > ranges[r].1
                {
                    ranges[r].1 = ranges[c].1;
                    ranges.remove(c);
                    modified = true;
                    break;
                }
                // encompassing
                if ranges[c].0 < ranges[r].0 && ranges[c].1 > ranges[r].1 {
                    modified = true;
                    ranges.remove(r);
                    break;
                }
                // encompassed
                if ranges[c].0 >= ranges[r].0
                    && ranges[c].0 <= ranges[r].1
                    && ranges[c].1 <= ranges[r].1
                    && ranges[c].1 >= ranges[r].0
                {
                    modified = true;
                    ranges.remove(c);
                    break;
                }
            }
            if modified {
                break;
            }
        }
    }
    let mut total_ids: usize = 0;
    for range in ranges {
        if range.0 == range.1 {
            total_ids += 1;
            continue;
        }
        total_ids += range.1 - range.0 + 1;
    }
    Some(total_ids)
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
        assert_eq!(result, Some(14));
    }
}
