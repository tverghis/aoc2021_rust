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
    // 8-element array, where the element at a given index represents the number of fish
    // at that age.
    // i.e., the value at index 2 represents the number of fish with age 2.
    let mut age_index = get_age_index(input);

    for _ in 0..num_days {
        age_index = tick(&age_index);
    }

    age_index.iter().sum()
}

fn get_age_index(input: &[u32]) -> [usize; 9] {
    let mut age_index = [0usize; 9];

    for &n in input.iter() {
        let n = n as usize;
        age_index[n] = age_index[n] + 1;
    }

    age_index
}

fn tick(age_index: &[usize; 9]) -> [usize; 9] {
    let num_spawns = age_index[0];
    let mut new_idx = [0; 9];

    for i in 0..8 {
        new_idx[i] = age_index[i + 1];
    }

    new_idx[8] = num_spawns;
    new_idx[6] = new_idx[6] + num_spawns;

    new_idx
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

    #[test]
    fn test_get_age_index() {
        assert_eq!(
            get_age_index(&vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],),
            [2, 1, 0, 1, 1, 4, 3, 3, 2]
        );
    }

    #[test]
    fn test_tick() {
        let mut age_index = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        assert_eq!(tick(&mut age_index), [1, 1, 2, 1, 0, 0, 0, 0, 0]);
    }
}
