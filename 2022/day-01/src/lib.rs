
pub fn process_part_1(input: &str) -> i32 {
    input.split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part_1(input);
        assert_eq!(result, 24000);
    }
}
