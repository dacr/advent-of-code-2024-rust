mod day07 {
    use regex::Regex;

    // ------------------------------------------------------------------------------
    type Num = u64;

    #[derive(Eq, PartialEq, Debug)]
    struct Equation {
        result: Num,
        values: Vec<Num>,
    }

    fn parse(input: String) -> Vec<Equation> {
        let mut equations: Vec<Equation> = vec![];
        for line in input.lines() {
            let mut parts = line.trim().splitn(2, ": ");

            let result: Num = match parts.next() {
                None => 0,
                Some(value) => value.parse().unwrap(),
            };

            let values: Vec<Num> = match parts.next() {
                None => vec![],
                Some(rawValues) => rawValues
                    .split(" ")
                    .map(|value| value.parse().unwrap())
                    .collect(),
            };

            equations.push(Equation { result, values });
        }
        equations
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn has_solution_with_two_operators_worker(
        result: Num,
        computed: Num,
        remaining_values: &[Num],
    ) -> bool {
        if (computed > result) {
            false
        } else if (remaining_values.len() == 0) {
            computed == result
        } else {
            has_solution_with_two_operators_worker(
                result,
                computed + remaining_values.first().unwrap(),
                &remaining_values[1..],
            ) || has_solution_with_two_operators_worker(
                result,
                computed * remaining_values.first().unwrap(),
                &remaining_values[1..],
            )
        }
    }

    fn has_solution_with_two_operators(equation: &Equation) -> bool {
        match equation.values.first() {
            None => false,
            Some(head) => {
                let tail = &equation.values[1..];
                has_solution_with_two_operators_worker(equation.result, *head, tail)
            }
        }
    }

    fn resolve_star1(input: String) -> Num {
        let equations = parse(input);
        equations
            .iter().filter(|eq| has_solution_with_two_operators(eq))
            .map(|eq| eq.result)
            .sum()
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn has_solution_with_three_operators_worker(
        result: Num,
        computed: Num,
        remaining_values: &[Num],
    ) -> bool {
        if (computed > result) {
            false
        } else if (remaining_values.len() == 0) {
            computed == result
        } else {
            has_solution_with_three_operators_worker(
                result,
                computed + remaining_values.first().unwrap(),
                &remaining_values[1..],
            ) || has_solution_with_three_operators_worker(
                result,
                computed * remaining_values.first().unwrap(),
                &remaining_values[1..],
            ) || has_solution_with_three_operators_worker(
                result,
                concat(computed, *remaining_values.first().unwrap()),
                &remaining_values[1..],
            )
        }
    }

    fn concat(a: Num, b: Num) -> Num {
        let mut tmp = a.to_string();
        tmp.push_str(b.to_string().as_str());
        tmp.parse().unwrap()
    }

    fn has_solution_with_three_operators(equation: &Equation) -> bool {
        match equation.values.first() {
            None => false,
            Some(head) => {
                let tail = &equation.values[1..];
                has_solution_with_three_operators_worker(equation.result, *head, tail)
            }
        }
    }
    
    
    fn resolve_star2(input: String) -> Num {
        let equations = parse(input);
        equations
            .iter().filter(|eq| has_solution_with_three_operators(eq))
            .map(|eq| eq.result)
            .sum()
    }

    // =================================================================================================================
    #[cfg(test)]
    mod tests {
        use std::fs;

        fn read_content(filename: &str) -> String {
            fs::read_to_string(filename).unwrap()
        }

        #[test]
        fn result_star1_test() {
            let data1 = read_content("data/day07/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 3749);

            let data2 = read_content("data/day07/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 20665830408335);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day07/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 11387);

            let data2 = read_content("data/day07/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 354060705047464);
        }
    }
}
