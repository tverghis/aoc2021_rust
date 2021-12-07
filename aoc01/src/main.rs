const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let input = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", count_increases(&input));
    println!("Part 2: {}", count_increasing_sums(&input));
}

fn count_increases(input: &[u32]) -> usize {
    input.windows(2).filter(|nums| nums[1] > nums[0]).count()
}

fn count_increasing_sums(input: &[u32]) -> usize {
    let three_window_sums = input
        .windows(3)
        .map(|nums| nums.iter().sum())
        .collect::<Vec<u32>>();

    count_increases(&three_window_sums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(&[]), 0);
        assert_eq!(count_increases(&[1]), 0);
        assert_eq!(count_increases(&[2, 2]), 0);
        assert_eq!(count_increases(&[2, 1]), 0);
        assert_eq!(
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn test_count_increasing_sums() {
        assert_eq!(count_increasing_sums(&[]), 0);
        assert_eq!(count_increasing_sums(&[1]), 0);
        assert_eq!(count_increasing_sums(&[1, 2]), 0);
        assert_eq!(count_increasing_sums(&[1, 2, 3]), 0);
        assert_eq!(count_increasing_sums(&[1, 2, 3, 0]), 0);
        assert_eq!(count_increasing_sums(&[1, 2, 3, 1]), 0);
        assert_eq!(
            count_increasing_sums(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
