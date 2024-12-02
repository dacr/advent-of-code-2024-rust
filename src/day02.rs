mod day02 {
    use regex::Regex;

    // -----------------------------------------------------------------------------------------------------------------

    fn parse(lines: Vec<String>) -> Vec<Vec<i32>> {
        let sep = Regex::new(r" +").unwrap();
        lines
            .iter()
            .map(|line| line.trim())
            .map(|line| sep.split(line))
            .map(|splitted| splitted.into_iter().map(|s| s.parse().unwrap()).collect())
            .collect()
    }

    fn is_inc(a: i32, b: i32) -> bool {
        a < b && (b - a <= 3)
    }

    fn is_dec(a: i32, b: i32) -> bool {
        a > b && (a - b <= 3)
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn report_check(report: &Vec<i32>, check: fn(i32, i32) -> bool) -> bool {
        report.windows(2).all(|window| check(window[0], window[1]))
    }

    fn resolve_star1(lines: Vec<String>) -> i32 {
        let reports = parse(lines);
        reports
            .iter()
            .filter(|report| report_check(report, is_inc) || report_check(report, is_dec))
            .count() as i32
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn report_fixable_check(report: &Vec<i32>, check: fn(i32, i32) -> bool) -> bool {
        //let candidates = (0..report.len() - 1)
        //    .into_iter()
        //    .map(|i| report.clone().remove(i))
        //    .collect();
        //candidates
        true
    }

    fn resolve_star2(lines: Vec<String>) -> i32 {
        let reports = parse(lines);
        reports
            .iter()
            .filter(|report| {
                report_fixable_check(report, is_inc) || report_fixable_check(report, is_dec)
            })
            .count() as i32
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
            let data1 = read_lines("data/day02/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 2);

            let data2 = read_lines("data/day02/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 564);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_lines("data/day02/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 4);

            let data2 = read_lines("data/day02/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 604);
        }
    }
}
