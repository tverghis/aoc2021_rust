use std::str::Chars;

use itertools::{peek_nth, PeekNth};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut lines = INPUT.lines().collect::<Vec<_>>();

    let diag = DiagnosticReport::new(&mut lines);

    println!("Part 1: {}", diag.power_consumption());
}

struct DiagnosticReport {
    gamma_rate: usize,
    epsilon_rate: usize,
}

impl DiagnosticReport {
    fn new(input: &mut [&str]) -> Self {
        let line_length = input.get(0).unwrap().len();

        let mut lines = input
            .iter_mut()
            .map(|l| peek_nth(l.chars()))
            .collect::<Vec<_>>();

        let (gamma_rate, epsilon_rate) = get_gamma_epsilon_rates(&mut lines, line_length);

        DiagnosticReport {
            gamma_rate,
            epsilon_rate,
        }
    }

    fn power_consumption(&self) -> usize {
        self.gamma_rate * self.epsilon_rate
    }
}

fn get_gamma_epsilon_rates(mut input: &mut [PeekNth<Chars>], line_length: usize) -> (usize, usize) {
    let mut gamma_rate_string = String::new();
    let mut epsilon_rate_string = String::new();

    for i in 0..line_length {
        let bin_info = BinaryStringInfo::new_for_idx(&mut input, i);

        if bin_info.num_zeroes > bin_info.num_ones {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        } else {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        }
    }

    (
        usize::from_str_radix(&gamma_rate_string, 2).unwrap(),
        usize::from_str_radix(&epsilon_rate_string, 2).unwrap(),
    )
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
}
