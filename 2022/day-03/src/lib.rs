
pub fn process_part_1(input: &str) -> i32 {
    0
}

pub fn process_part_2(input: &str) -> i32 {
     0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

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