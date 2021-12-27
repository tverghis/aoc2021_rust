fn main() {
    let mut input = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    input.sort_unstable();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[u32]) -> u32 {
    // The median value gives us the necessary position, because
    // the property of the median is that it minimizes the distance
    // between the values in the set.
    // We assume that input is sorted.
    let pos = input[(input.len() + 1) / 2];
    cost_linear(input, pos)
}

fn part_2(input: &[u32]) -> u32 {
    let mut lowest_cost = u32::MAX;

    // Assume that the ideal position lies in [0, max(input)]
    for pos in 0..(*input.iter().max().unwrap()) {
        let total_cost = input.iter().fold(0, |acc, &x| acc + cost_nonlinear(x, pos));
        lowest_cost = u32::min(lowest_cost, total_cost);
    }

    lowest_cost
}

fn cost_linear(input: &[u32], pos: u32) -> u32 {
    input
        .iter()
        .fold(0, |acc, x| acc + (*x as i32 - pos as i32).abs() as u32)
}

fn cost_nonlinear(cur: u32, desired: u32) -> u32 {
    let diff = (cur as i32 - desired as i32).abs() as u32;
    // sum of natural numbers [1, diff]
    (diff * (diff + 1)) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(cost_linear(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 2), 37);
        assert_eq!(cost_linear(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 1), 41);
        assert_eq!(cost_linear(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 3), 39);
        assert_eq!(cost_linear(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 10), 71);
    }

    #[test]
    fn test_cost_nonlinear() {
        assert_eq!(cost_nonlinear(3, 5), 3);
        assert_eq!(cost_nonlinear(16, 5), 66);
        assert_eq!(cost_nonlinear(5, 5), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16]), 168);
    }
}
