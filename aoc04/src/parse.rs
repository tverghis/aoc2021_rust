use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, space0, space1},
    multi::{count, separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::board::Board;

pub fn parse_draws_and_boards(s: &str) -> IResult<&str, (Vec<u32>, Vec<Board<u32>>)> {
    separated_pair(parse_draws, multispace1, parse_boards)(s)
}

fn parse_draws(s: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(tag(","), digit1), newline)(s).map(map_result_str_list_to_u32_list)
}

fn parse_boards(s: &str) -> IResult<&str, Vec<Board<u32>>> {
    separated_list0(newline, parse_board)(s)
}

fn parse_board(s: &str) -> IResult<&str, Board<u32>> {
    count(parse_board_row, 5)(s).map(|(rem, rows)| (rem, Board::new(rows)))
}

fn parse_board_row(s: &str) -> IResult<&str, Vec<u32>> {
    terminated(preceded(space0, separated_list1(space1, digit1)), newline)(s)
        .map(map_result_str_list_to_u32_list)
}

fn map_result_str_list_to_u32_list<'a>((rem, v): (&'a str, Vec<&str>)) -> (&'a str, Vec<u32>) {
    (rem, v.iter().map(|s| s.parse().unwrap()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_draws() {
        assert_eq!(parse_draws("1,2,3,4\n"), Ok(("", vec![1, 2, 3, 4])));
    }

    #[test]
    fn test_parse_board_row() {
        assert_eq!(
            parse_board_row("22 13 17 11 0\n"),
            Ok(("", vec![22, 13, 17, 11, 0]))
        );
    }

    #[test]
    fn test_parse_full_board() {
        assert_eq!(
            parse_board(
                r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#
            ),
            Ok((
                "",
                Board::new(vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ])
            ))
        );
    }

    #[test]
    fn test_parse_multi_boards() {
        assert_eq!(
            parse_boards(
                r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#
            ),
            Ok((
                "",
                vec![
                    Board::new(vec![
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ]),
                    Board::new(vec![
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ]),
                    Board::new(vec![
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ])
                ]
            ))
        );
    }

    #[test]
    fn test_parse_draws_and_boards() {
        assert_eq!(
            parse_draws_and_boards(
                r#"1,2,3,4

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#
            ),
            Ok((
                "",
                (
                    vec![1, 2, 3, 4],
                    vec![
                        Board::new(vec![
                            vec![22, 13, 17, 11, 0],
                            vec![8, 2, 23, 4, 24],
                            vec![21, 9, 14, 16, 7],
                            vec![6, 10, 3, 18, 5],
                            vec![1, 12, 20, 15, 19],
                        ]),
                        Board::new(vec![
                            vec![3, 15, 0, 2, 22],
                            vec![9, 18, 13, 17, 5],
                            vec![19, 8, 7, 25, 23],
                            vec![20, 11, 10, 24, 4],
                            vec![14, 21, 16, 12, 6],
                        ]),
                        Board::new(vec![
                            vec![14, 21, 17, 24, 4],
                            vec![10, 16, 15, 9, 19],
                            vec![18, 8, 23, 26, 20],
                            vec![22, 11, 13, 6, 5],
                            vec![2, 0, 12, 3, 7],
                        ])
                    ]
                )
            ))
        );
    }
}
