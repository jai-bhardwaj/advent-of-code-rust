pub fn process_part_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i32>().unwrap()).sum::<i32>())
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

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn process_part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 45000);
    }
}
