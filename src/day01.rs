mod day01 {
    use regex::Regex;

    fn parse(lines: Vec<String>) -> Vec<Vec<i32>> {
        let sep = Regex::new(r" +").unwrap();
        lines
            .iter()
            .map(|line| line.trim())
            .map(|line| sep.splitn(line, 2))
            .map(|splitted| splitted.into_iter().map(|s| s.parse().unwrap()).collect())
            .collect()
    }

    // -----------------------------------------------------------------------------------------------------------------
    fn resolve_star1(lines: Vec<String>) -> i32 {
        let data = parse(lines);
        let mut left: Vec<i32> = data.iter().map(|row| row[0]).collect();
        left.sort();
        let mut right: Vec<i32> = data.iter().map(|row| row[1]).collect();
        right.sort();
        let diff = left.iter().zip(right.iter()).map(|(&a, &b)| (a - b).abs());
        diff.sum()
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn resolve_star2(lines: Vec<String>) -> i32 {
        let data = parse(lines);
        let left: Vec<i32> = data.iter().map(|row| row[0]).collect();
        let right: Vec<i32> = data.iter().map(|row| row[1]).collect();

        left.iter()
            .map(|n| n * (right.iter().filter(|o| n.eq(o)).count() as i32))
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
            let data1 = read_lines("data/day01/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 31);

            let data2 = read_lines("data/day01/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 27647262);
        }
    }
}
