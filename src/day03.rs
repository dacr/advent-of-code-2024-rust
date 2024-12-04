mod day03 {
    use regex::{Matches, Regex};

    // -----------------------------------------------------------------------------------------------------------------

    fn resolve_star1(content: String) -> i32 {
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        mul_re
            .captures_iter(content.as_str())
            .map(|caps| match caps.extract() {
                (_, [a, b]) => a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap(),
            })
            .sum()
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn resolve_star2(content: String) -> i32 {
        let instr_re = Regex::new(r"(?:(mul)\(\d+,\d+\))|(?:(do)\(\))|(?:(don't)\(\))").unwrap();
        let instrs = instr_re.find_iter(content.as_str());

        fn compute(mut remain: Matches, current: i32, doit: bool) -> i32 {
            let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            match remain.next() {
                Some(m) if m.as_str().contains("mul") && doit => {
                    match mul_re.captures(m.as_str()).map(|c| c.extract()) {
                        None => compute(remain, current, doit),
                        Some((_, [a, b])) => compute(
                            remain,
                            current + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap(),
                            doit,
                        ),
                    }
                }
                Some(m) if m.as_str().contains("mul") => compute(remain, current, doit),
                Some(m) if m.as_str().contains("don't") => compute(remain, current, false),
                Some(m) if m.as_str().contains("do") => compute(remain, current, true),
                Some(_) => compute(remain, current, doit),
                None => current,
            }
        }

        compute(instrs, 0, true)
    }

    // =================================================================================================================
    #[cfg(test)]
    mod tests {
        use std::fs;

        // fn read_lines(filename: &str) -> Vec<String> {
        //     fs::read_to_string(filename)
        //         .unwrap()
        //         .lines()
        //         .map(String::from)
        //         .collect()
        // }

        fn read_content(filename: &str) -> String {
            fs::read_to_string(filename).unwrap()
        }

        #[test]
        fn result_star1_test() {
            let data1 = read_content("data/day03/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 161);

            let data2 = read_content("data/day03/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 188741603);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day03/example-2.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 48);

            let data2 = read_content("data/day03/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 67269798);
        }
    }
}
