mod day01 {

    fn find_first_digit(input: &String) -> u32 {
        let result = input
            .find(|c: char| c.is_digit(10))
            .map(|i: usize| input.chars().nth(i).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap_or(0);
        // println!("first={v}", v = result);
        result
    }

    fn find_last_digit(input: &String) -> u32 {
        let result = input
            .rfind(|c: char| c.is_digit(10))
            .map(|i: usize| input.chars().nth(i).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap_or(0);
        // println!("last={v}", v = result);
        result
    }

    fn resolve_star1(lines: Vec<String>) -> i32 {
        lines
            .iter()
            .map(|line| (find_first_digit(line) * 10 + find_last_digit(line)) as i32)
            .sum()
    }
    // -----------------------------------------------------------------------------------------------------------------

    use phf::phf_map;

    static FIXES: phf::Map<&'static str, i32> = phf_map! {
        "one"=>1,
        "two"=>2,
        "three"=>3,
        "four"=>4,
        "five"=>5,
        "six"=>6,
        "seven"=>7,
        "eight"=>8,
        "nine"=>9,
    };

    fn find_first(input: &str) -> i32 {
        match FIXES
            .into_iter()
            .find(|(&key, &value)| input.starts_with(&key))
        {
            Some((&key, &num)) => num,
            None if input.chars().nth(0).filter(|ch| ch.is_digit(10)).is_some() => {
                input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32
            }
            None => find_first(&input[1..]),
        }
    }
    fn find_last(input: &str) -> i32 {
        match FIXES
            .into_iter()
            .rfind(|(&key, &value)| input.starts_with(&key))
        {
            Some((&key, &num)) => num,
            None if input.chars().nth(0).filter(|ch| ch.is_digit(10)).is_some() => {
                input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32
            }
            None => find_last(&input[1..]),
        }
    }

    fn resolve_star2(lines: Vec<String>) -> i32 {
        lines
            .iter()
            .map(|line| (find_first(line) * 10 + find_last(line)) as i32)
            .sum()
    }

    // =================================================================================================================
    #[cfg(test)]
    mod tests {
        use std::fs;

        fn read_lines(filename: &str) -> Vec<String> {
            fs::read_to_string(filename)
                .unwrap()
                .lines()
                .map(String::from)
                .collect()
        }

        #[test]
        fn result_star1_test() {
            let data1 = read_lines("data/day01/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 11);

            let data2 = read_lines("data/day01/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 2344935);
        }
        #[test]
        fn result_star2_test() {
            let data1 = read_lines("data/day01/example-2.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 31);

            let data2 = read_lines("data/day01/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 27647262);
        }
    }
}
