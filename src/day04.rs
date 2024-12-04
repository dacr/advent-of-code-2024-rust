mod day04 {
    use regex::{Captures, Match, Matches, Regex};

    struct Coord {
        x: i32,
        y: i32,
    }

    impl Coord {
        fn ne
    }

    struct Cell {
        letter : char
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn resolve_star1(content: String) -> i32 {
        0
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn resolve_star2(content: String) -> i32 {
        0
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

        fn read_content(filename: &str) -> String {
            fs::read_to_string(filename).unwrap()
        }

        #[test]
        fn result_star1_test() {
            let data1 = read_content("data/day04/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 18);

            let data2 = read_content("data/day04/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 2551);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day04/example-2.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 9);

            let data2 = read_content("data/day04/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 1985);
        }
    }
}
