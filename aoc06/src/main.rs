fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[u32]) -> usize {
    fish_population_after_days(input, 80)
}

fn part_2(input: &[u32]) -> usize {
    fish_population_after_days(input, 256)
}

fn fish_population_after_days(input: &[u32], num_days: u32) -> usize {
    let mut input: Vec<u32> = input.iter().copied().collect();

    for _ in 0..num_days {
        let mut num_spawned = 0;
        for n in input.iter_mut() {
            if *n == 0 {
                *n = 6;
                num_spawned += 1;
            } else {
                *n -= 1;
            }
        }

        input.extend(vec![8; num_spawned]);
    }

    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fish_population() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(fish_population_after_days(&input, 18), 26);
        assert_eq!(fish_population_after_days(&input, 80), 5934);
    }
}
