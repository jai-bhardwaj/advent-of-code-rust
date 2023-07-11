use std::collections::HashSet;


fn priority(chr: char) -> usize {
    if chr.is_lowercase() {
        chr as usize - 'a' as usize + 1
    }else {
        chr as usize - 'A' as usize + 27
    } 
}

pub fn process_part_1(input: &str) -> usize {
    // let mut ans_array = Vec::<char>::new();
    let mut ans = 0;
    for line in input.lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let mut hash_set_1 = HashSet::new();
        let mut hash_set_2 = HashSet::new();
        hash_set_1.extend(first_half.chars());
        hash_set_2.extend(second_half.chars());
        let both = hash_set_1.intersection(&hash_set_2).next().unwrap();
        ans += priority(*both);
    }
    ans

}

pub fn process_part_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut ans = 0;
    let mut i = 0;
    while i < lines.len() {
        
        let mut hash_set_1 = HashSet::new();
        let mut hash_set_2 = HashSet::new();
        let mut hash_set_3 = HashSet::new();
        hash_set_1.extend(lines[i].chars());
        hash_set_2.extend(lines[i+1].chars());
        hash_set_3.extend(lines[i+2].chars());
        let both = hash_set_1.intersection(&hash_set_2);
        let mut hash_set = HashSet::new();
        for ch in both {
            hash_set.insert(*ch);
        }
        let both = hash_set_3.intersection(&hash_set).next().unwrap();
        ans += priority(*both);
        i+=3;
    }
    ans
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
        assert_eq!(result, 157);
    }

    // #[ignore]
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 7);
    }
}
