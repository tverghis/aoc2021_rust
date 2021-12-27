fn main() {
    let mut input = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    input.sort_unstable();

    println!("Part 1: {}", part_1(&input));
}

fn part_1(input: &[u32]) -> u32 {
    // The median value gives us the necessary answer, because
    // the property of the median is that it minimizes the distance
    // between the values in the set.
    // We assume that input is sorted.
    let pos = input[(input.len() + 1) / 2];
    align_to_position(input, pos)
}

fn align_to_position(input: &[u32], pos: u32) -> u32 {
    input
        .iter()
        .fold(0, |acc, x| acc + (*x as i32 - pos as i32).abs() as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(align_to_position(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 2), 37);
        assert_eq!(align_to_position(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 1), 41);
        assert_eq!(align_to_position(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 3), 39);
        assert_eq!(align_to_position(&[0, 1, 1, 2, 2, 2, 4, 7, 14, 16], 10), 71);
    }
}
