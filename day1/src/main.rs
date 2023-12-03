use std::fs;

fn main() {
    let file: String = fs::read_to_string("input").expect("unable to read file");
    process_part1(file);
}

fn process_part1(input: String) -> u32 {
    let num_lines = get_num_lines(input);
    let mut sum = 0;
    for line in num_lines {
        let num = format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap());
        sum += num.parse::<u32>().unwrap();
    }
    sum
}

fn get_num_lines(input: String) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| {
            let nums: String = line.chars().filter(|c| c.is_numeric()).collect();
            if nums.is_empty() {
                None
            } else {
                Some(nums)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(super::process_part1(input.to_string()), 142);
    }
}
