const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let input = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", count_increases(&input));
}

fn count_increases(input: &[u32]) -> usize {
    input.windows(2).filter(|nums| nums[1] > nums[0]).count()
}

#[cfg(test)]
mod test {
    use super::count_increases;

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
}
