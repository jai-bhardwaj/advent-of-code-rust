enum Move {
    Rock= 1,
    Paper,
    Scissors,
}

pub fn process_part_1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|moves| {
            let (op_move, my_move) = moves.split_once(' ').unwrap();
            println!("{} vs {}", op_move, my_move);

            let my_move  = match my_move {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => unreachable!(),
            };
            let op_move = match op_move {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => unreachable!(),
            };

            let mut score = match my_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            };

            score += match (op_move, my_move) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            };

            score
        }).sum::<i32>()
}

pub fn process_part_2(input: &str) -> i32 {
     input
        .split("\n")
        .map(|moves| {
            let (op_move, my_move) = moves.split_once(' ').unwrap();
            // println!("{} vs {}", op_move, my_move);

            // let my_move_expected = my_move.clone();
            // let my_move  = match my_move {
            //     "X" => Move::Rock,
            //     "Y" => Move::Paper,
            //     "Z" => Move::Scissors,
            //     _ => unreachable!(),
            // };
            let op_move = match op_move {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => unreachable!(),
            };

            let score = match op_move {
                Move::Rock => match my_move {
                        "X" => 0+3,
                        "Y" => 3+1,
                        "Z" => 6+2,
                        _ => unreachable!(),
                    },
                Move::Paper => match my_move {
                    "X" => 0+1,
                    "Y" => 3+2,
                    "Z" => 6+3,
                    _ => unreachable!(),
                },
                Move::Scissors => match my_move {
                    "X" => 0+2,
                    "Y" => 3+3,
                    "Z" => 6+1,
                    _ => unreachable!(),
                },
            };

            score
        }).sum::<i32>()

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn it_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 15);
    }

    #[ignore]
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 12);
    }
}
