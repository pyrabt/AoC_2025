advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage_total: u64 = 0;
    for line in input.lines() {
        let batteries: Vec<&str> = line.trim().split("").filter(|x| *x != "").collect();
        let mut lt_batt = "";
        let mut lt_batt_index = 0;
        let mut rt_batt = "";
        for lb_index in 0..batteries.len() - 1 {
            if batteries[lb_index] > lt_batt {
                lt_batt = batteries[lb_index];
                lt_batt_index = lb_index;
            }
            if lt_batt == "9" {
                break;
            }
        }
        for rb_index in lt_batt_index + 1..batteries.len() {
            if batteries[rb_index] > rt_batt {
                rt_batt = batteries[rb_index]
            }
            if rt_batt == "9" {
                break;
            }
        }
        joltage_total += format!("{}{}", lt_batt, rt_batt).parse::<u64>().unwrap();
    }
    Some(joltage_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage_total: u64 = 0;
    for line in input.lines() {
        let mut remaining_skips = line.len() - 12;
        let mut current_joltage: Vec<&str> = vec![];
        let batteries: Vec<&str> = line.trim().split("").filter(|x| *x != "").collect();

        let mut start_index = 0;
        for _ in 0..12 {
            let mut h_batt = "";
            let mut h_batt_index = start_index;
            let mut read_len = start_index + 1 + remaining_skips;
            if read_len > batteries.len() {
                read_len = batteries.len();
            }
            for b_index in start_index..read_len {
                if batteries[b_index] > h_batt {
                    h_batt = batteries[b_index];
                    h_batt_index = b_index;
                    if h_batt == "9" {
                        break;
                    }
                }
            }
            current_joltage.push(h_batt);
            remaining_skips -= h_batt_index - start_index;
            start_index = h_batt_index + 1;
        }
        joltage_total += current_joltage.join("").parse::<u64>().unwrap();
    }
    Some(joltage_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
