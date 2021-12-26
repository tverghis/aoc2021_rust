use std::ops::RangeInclusive;

use nom::{
    bytes::complete::{is_a, tag},
    sequence::separated_pair,
    IResult,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point(u32, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
    DiagonalPrimary,
    DiagonalSecondary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LineSegment {
    endpoints: (Point, Point),
    pub orientation: Orientation,
}

impl LineSegment {
    fn new(endpoints: (Point, Point)) -> Self {
        let Point(x1, _) = endpoints.0;
        let Point(x2, _) = endpoints.1;

        // Ensure that the endpoints are "sorted" by their x-coords.
        // This makes some calculations easier later on.
        let endpoints = if x1 <= x2 {
            endpoints
        } else {
            (endpoints.1, endpoints.0)
        };

        // Re-bind x1, x2, y1, y2 in case the order of the endpoints changed.
        let Point(x1, y1) = endpoints.0;
        let Point(x2, y2) = endpoints.1;

        let orientation = if x1 == x2 {
            Orientation::Vertical
        } else if y1 == y2 {
            Orientation::Horizontal
        } else if y2 > y1 {
            Orientation::DiagonalSecondary
        } else {
            Orientation::DiagonalPrimary
        };

        LineSegment {
            endpoints,
            orientation,
        }
    }

    pub fn points(&self) -> Vec<Point> {
        let (Point(x1, y1), Point(x2, y2)) = self.endpoints;

        match self.orientation {
            Orientation::Horizontal => RangeInclusive::new(x1, x2).map(|x| Point(x, y1)).collect(),
            Orientation::Vertical => {
                let start = std::cmp::min(y1, y2);
                let end = std::cmp::max(y1, y2);

                RangeInclusive::new(start, end)
                    .map(|y| Point(x1, y))
                    .collect()
            }
            Orientation::DiagonalPrimary => {
                let mut x = x1;
                let mut y = y1 as i32;
                let mut points = vec![];

                while (x <= x2) && (y as u32 >= y2) {
                    points.push(Point(x, y as u32));

                    x += 1;
                    y -= 1;
                }

                points
            }
            Orientation::DiagonalSecondary => {
                let mut x = x1;
                let mut y = y1;
                let mut points = vec![];

                while (x <= x2) && (y <= y2) {
                    points.push(Point(x, y));

                    x += 1;
                    y += 1;
                }

                points
            }
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(parse_u32, tag(","), parse_u32)(input).map(|(rem, (x, y))| (rem, Point(x, y)))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    is_a("1234567890")(input).map(|(rem, num)| (rem, num.parse::<u32>().unwrap()))
}

pub fn parse_line_segment(input: &str) -> LineSegment {
    separated_pair(parse_point, tag(" -> "), parse_point)(input)
        .map(|(_, (p1, p2))| LineSegment::new((p1, p2)))
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("0,9"), Ok(("", Point(0, 9))));
        assert_eq!(parse_point("12,545"), Ok(("", Point(12, 545))));
    }

    #[test]
    fn test_parse_line_segment() {
        assert_eq!(
            parse_line_segment("477,485 -> 864,485"),
            LineSegment::new((Point(477, 485), Point(864, 485)))
        );
        assert_eq!(
            parse_line_segment("35,10 -> 20,120"),
            LineSegment::new((Point(20, 120), Point(35, 10)))
        );
    }

    #[test]
    fn test_orientation() {
        assert_eq!(
            parse_line_segment("0,9 -> 5,9").orientation,
            Orientation::Horizontal
        );
        assert_eq!(
            parse_line_segment("2,2 -> 2,1").orientation,
            Orientation::Vertical
        );
        assert_eq!(
            parse_line_segment("8,0 -> 0,8").orientation,
            Orientation::DiagonalPrimary
        );
        assert_eq!(
            parse_line_segment("1,0 -> 5,4").orientation,
            Orientation::DiagonalSecondary
        );
    }

    #[test]
    fn test_points_horizontal() {
        let line_segment = parse_line_segment("2,2 -> 10,2");
        assert_eq!(
            line_segment.points(),
            vec![
                Point(2, 2),
                Point(3, 2),
                Point(4, 2),
                Point(5, 2),
                Point(6, 2),
                Point(7, 2),
                Point(8, 2),
                Point(9, 2),
                Point(10, 2),
            ]
        );
    }

    #[test]
    fn test_points_vertical() {
        let line_segment = parse_line_segment("2,2 -> 2,10");
        assert_eq!(
            line_segment.points(),
            vec![
                Point(2, 2),
                Point(2, 3),
                Point(2, 4),
                Point(2, 5),
                Point(2, 6),
                Point(2, 7),
                Point(2, 8),
                Point(2, 9),
                Point(2, 10),
            ]
        );
    }

    #[test]
    fn test_points_diagonal_primary() {
        let line_segment = parse_line_segment("1,1 -> 3,3");
        assert_eq!(
            line_segment.points(),
            vec![Point(1, 1), Point(2, 2), Point(3, 3)]
        );
    }

    #[test]
    fn test_points_diagonal_secondary() {
        let line_segment = parse_line_segment("9,7 -> 7,9");
        assert_eq!(
            line_segment.points(),
            vec![Point(7, 9), Point(8, 8), Point(9, 7)]
        );
    }
}
