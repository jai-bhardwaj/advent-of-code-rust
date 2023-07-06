enum Move {
    Rock= 1,
    Paper,
    Scissors,
}

pub fn process_part_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
            .map(|y| {
                let opponent_option = y.chars().nth(0).unwrap();
                let my_option = y.chars().nth(2).unwrap();

                let mut score = 0;

                let op_move: Move = match opponent_option {
                    'A' => Move::Rock,
                    'B' => Move::Paper,
                    'C' => Move::Scissors,
                    _ => unreachable!()
                };

                let my_move = match my_option {
                    'X' => Move::Rock,
                    'Y' => Move::Paper,
                    'Z' => Move::Scissors,
                    _ => unreachable!(),
                };

                score += &my_move as i32;

                match &my_move {
                    Move::Rock => {
                        if op_move as i32 == Move::Scissors as i32 {
                            score += 6;
                        }
                    },
                    Move::Paper => {
                        if op_move as i32 == Move::Rock as i32 {
                            score += 6;
                        }
                    },
                    Move::Scissors => {
                        if op_move as i32 == Move::Paper as i32 {
                            score += 6;
                        }
                    },
                }
                score
                
            })
            .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn process_part_2(input: &str) -> i32 {
    let mut res = input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    res.sort_by(|a,b| b.cmp(a));
    let res: i32 = res.iter().take(3).sum();
    res

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 15);
    }

    #[ignore]
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 45000);
    }
}
