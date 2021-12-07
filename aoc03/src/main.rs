use std::cmp::Ordering;
use std::str::Chars;

use itertools::{peek_nth, PeekNth};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut lines = INPUT.lines().collect::<Vec<_>>();

    let diag = DiagnosticReport::new(&mut lines);

    println!("Part 1: {}", diag.power_consumption());
    println!("Part 2: {}", diag.life_support_rating());
}

struct DiagnosticReport {
    gamma_rate: usize,
    epsilon_rate: usize,
    o2_gen_rating: usize,
    co2_scrubber_rating: usize,
}

impl DiagnosticReport {
    fn new(input: &mut [&str]) -> Self {
        let line_length = input.get(0).unwrap().len();

        let mut lines = input
            .iter_mut()
            .map(|l| peek_nth(l.chars()))
            .collect::<Vec<_>>();

        let (gamma_rate, epsilon_rate) = get_gamma_epsilon_rates(&mut lines, line_length);
        let (o2_gen_rating, co2_scrubber_rating) = get_o2_co2_ratings(&mut lines);

        DiagnosticReport {
            gamma_rate,
            epsilon_rate,
            o2_gen_rating,
            co2_scrubber_rating,
        }
    }

    fn power_consumption(&self) -> usize {
        self.gamma_rate * self.epsilon_rate
    }

    fn life_support_rating(&self) -> usize {
        self.o2_gen_rating * self.co2_scrubber_rating
    }
}

fn get_gamma_epsilon_rates(input: &mut [PeekNth<Chars>], line_length: usize) -> (usize, usize) {
    let mut gamma_rate_string = String::new();
    let mut epsilon_rate_string = String::new();

    for i in 0..line_length {
        let bin_info = BinaryStringInfo::new_for_idx(input, i);

        if bin_info.num_zeroes > bin_info.num_ones {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        } else {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        }
    }

    (
        usize_from_binary(&gamma_rate_string),
        usize_from_binary(&epsilon_rate_string),
    )
}

enum LsrBitCriteria {
    MostCommon,
    LeastCommon,
}

fn get_o2_co2_ratings(input: &mut [PeekNth<Chars>]) -> (usize, usize) {
    (
        get_lsr_rating_component(input, 0, LsrBitCriteria::MostCommon),
        get_lsr_rating_component(input, 0, LsrBitCriteria::LeastCommon),
    )
}

fn get_lsr_rating_component(
    input: &mut [PeekNth<Chars>],
    cur_idx: usize,
    bit_crit: LsrBitCriteria,
) -> usize {
    if input.len() == 1 {
        return usize_from_binary(&input.get_mut(0).unwrap().collect::<String>());
    }

    let bin_info = BinaryStringInfo::new_for_idx(input, cur_idx);

    let most_common_bit: u8 = match bin_info.num_zeroes.cmp(&bin_info.num_ones) {
        Ordering::Less | Ordering::Equal => 1,
        Ordering::Greater => 0,
    };

    let char_to_keep = (match bit_crit {
        LsrBitCriteria::MostCommon => most_common_bit,
        LsrBitCriteria::LeastCommon => most_common_bit ^ 0b0000_0001,
    } + b'0') as char;

    let mut filtered_input: Vec<PeekNth<Chars>> = input
        .iter_mut()
        .filter_map(|line| {
            if *line.peek_nth(cur_idx).unwrap() == char_to_keep {
                Some(line.clone())
            } else {
                None
            }
        })
        .collect();

    get_lsr_rating_component(&mut filtered_input, cur_idx + 1, bit_crit)
}

fn usize_from_binary(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct BinaryStringInfo {
    num_zeroes: usize,
    num_ones: usize,
}

impl BinaryStringInfo {
    fn new_for_idx(input: &mut [PeekNth<Chars>], n: usize) -> Self {
        let mut num_zeroes = 0;
        let mut num_ones = 0;

        for i in input.iter_mut() {
            if *i.peek_nth(n).unwrap() == '0' {
                num_zeroes += 1;
            } else {
                num_ones += 1;
            }
        }

        BinaryStringInfo {
            num_zeroes,
            num_ones,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_string_info() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| peek_nth(l.chars()))
        .collect::<Vec<_>>();

        assert_eq!(
            BinaryStringInfo::new_for_idx(&mut input, 0),
            BinaryStringInfo {
                num_zeroes: 5,
                num_ones: 7
            }
        );
        assert_eq!(
            BinaryStringInfo::new_for_idx(&mut input, 4),
            BinaryStringInfo {
                num_zeroes: 7,
                num_ones: 5
            }
        );
    }

    #[test]
    fn test_get_gamma_epsilon_rates() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| peek_nth(l.chars()))
        .collect::<Vec<_>>();

        assert_eq!(get_gamma_epsilon_rates(&mut input, 5), (22, 9));
    }

    #[test]
    fn test_power_consumption() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let diag = DiagnosticReport::new(&mut input);

        assert_eq!(diag.power_consumption(), 198);
    }

    #[test]
    fn test_get_o2_gen_rating() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| peek_nth(l.chars()))
        .collect::<Vec<_>>();

        assert_eq!(
            get_lsr_rating_component(&mut input, 0, LsrBitCriteria::MostCommon),
            23,
        );
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| peek_nth(l.chars()))
        .collect::<Vec<_>>();

        assert_eq!(
            get_lsr_rating_component(&mut input, 0, LsrBitCriteria::LeastCommon),
            10
        );
    }

    #[test]
    fn test_life_support_rating() {
        let mut input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let diag = DiagnosticReport::new(&mut input);

        assert_eq!(diag.life_support_rating(), 230);
    }
}
