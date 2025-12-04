advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let shelf: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split("").filter(|x| *x != "").collect())
        .collect();
    let max_h_index = shelf.len() - 1;
    let max_w_index = shelf[0].len() - 1;
    for h in 0..shelf.len() {
        for w in 0..shelf[0].len() {
            if shelf[h][w] == "." {
                continue;
            }
            let mut adjacent_rolls = 0;
            if h > 0 && shelf[h - 1][w] == "@" {
                adjacent_rolls += 1;
            }
            if h > 0 && w > 0 && shelf[h - 1][w - 1] == "@" {
                adjacent_rolls += 1;
            }
            if w > 0 && shelf[h][w - 1] == "@" {
                adjacent_rolls += 1;
            }
            if h < max_h_index && w > 0 && shelf[h + 1][w - 1] == "@" {
                adjacent_rolls += 1;
            }
            if h < max_h_index && shelf[h + 1][w] == "@" {
                adjacent_rolls += 1;
            }
            if h < max_h_index && w < max_w_index && shelf[h + 1][w + 1] == "@" {
                adjacent_rolls += 1;
            }
            if w < max_w_index && shelf[h][w + 1] == "@" {
                adjacent_rolls += 1;
            }
            if h > 0 && w < max_w_index && shelf[h - 1][w + 1] == "@" {
                adjacent_rolls += 1;
            }

            if adjacent_rolls < 4 {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let mut shelf: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split("").filter(|x| *x != "").collect())
        .collect();
    let max_h_index = shelf.len() - 1;
    let max_w_index = shelf[0].len() - 1;
    loop {
        let mut rolls_to_remove: Vec<(usize, usize)> = vec![];
        let mut loop_total: u64 = 0;
        for h in 0..shelf.len() {
            for w in 0..shelf[0].len() {
                if shelf[h][w] == "." {
                    continue;
                }
                let mut adjacent_rolls = 0;
                if h > 0 && shelf[h - 1][w] == "@" {
                    adjacent_rolls += 1;
                }
                if h > 0 && w > 0 && shelf[h - 1][w - 1] == "@" {
                    adjacent_rolls += 1;
                }
                if w > 0 && shelf[h][w - 1] == "@" {
                    adjacent_rolls += 1;
                }
                if h < max_h_index && w > 0 && shelf[h + 1][w - 1] == "@" {
                    adjacent_rolls += 1;
                }
                if h < max_h_index && shelf[h + 1][w] == "@" {
                    adjacent_rolls += 1;
                }
                if h < max_h_index && w < max_w_index && shelf[h + 1][w + 1] == "@" {
                    adjacent_rolls += 1;
                }
                if w < max_w_index && shelf[h][w + 1] == "@" {
                    adjacent_rolls += 1;
                }
                if h > 0 && w < max_w_index && shelf[h - 1][w + 1] == "@" {
                    adjacent_rolls += 1;
                }

                if adjacent_rolls < 4 {
                    rolls_to_remove.push((h, w));
                    loop_total += 1;
                }
            }
        }
        if loop_total == 0 {
            break;
        }
        for roll in rolls_to_remove {
            shelf[roll.0][roll.1] = ".";
        }
        total += loop_total;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
