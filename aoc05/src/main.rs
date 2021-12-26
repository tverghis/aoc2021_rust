use std::collections::HashMap;

use line_segment::{parse_line_segment, LineSegment};

use crate::line_segment::Orientation;

mod line_segment;

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part_1(&input));
}

fn parse_input(input: &str) -> Vec<LineSegment> {
    input.lines().map(parse_line_segment).collect()
}

fn part_1(input: &[LineSegment]) -> usize {
    let points = input
        .iter()
        .filter(|ls| {
            ls.orientation == Orientation::Horizontal || ls.orientation == Orientation::Vertical
        })
        .flat_map(|ls| ls.points());

    let mut contained_points = HashMap::new();

    for point in points {
        let prev_point = contained_points.entry(point).or_insert(0);
        *prev_point += 1;
    }

    contained_points.retain(|_, count| *count >= 2);

    contained_points.keys().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input_parsed = parse_input(input);

        assert_eq!(part_1(&input_parsed), 5);
    }
}
