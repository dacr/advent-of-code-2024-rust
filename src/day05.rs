mod day05 {
    use clap::builder::TypedValueParser;
    use regex::Regex;
    use std::collections::HashMap;
    use std::str::{Chars, SplitN};

    // ------------------------------------------------------------------------------
    #[derive(Eq, PartialEq, Debug)]
    struct PageOrderingRule {
        before: i32,
        after: i32,
    }
    impl PageOrderingRule {
        fn new(before: i32, after: i32) -> PageOrderingRule {
            PageOrderingRule { before, after }
        }
    }

    type PageOrderingRules = Vec<PageOrderingRule>;
    type PagesToProduce = Vec<i32>;
    type PagesToProduceList = Vec<PagesToProduce>;

    fn parse(input: String) -> (PageOrderingRules, PagesToProduceList) {
        let ord_re = Regex::new(r"(\d+)[|](\d+)").unwrap();
        let mut parts = input.trim().splitn(2, "\n\n");
        let page_ordering_rules: PageOrderingRules = match parts.next() {
            None => vec![],
            Some(orders) => ord_re
                .captures_iter(orders)
                .map(|found| match found.extract() {
                    (_, [a, b]) => PageOrderingRule::new(a.parse().unwrap(), b.parse().unwrap()),
                })
                .collect(),
        };

        let page_to_produces: PagesToProduceList = match parts.next() {
            None => vec![],
            Some(produces) => produces
                .lines()
                .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
                .collect(),
        };

        (page_ordering_rules, page_to_produces)
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn check_ordering(pages: &PagesToProduce, rules: &PageOrderingRules) -> bool {
        rules.iter().all(|rule| {
            let before = pages.iter().position(|&page| page == rule.before);
            let after = pages.iter().position(|&page| page == rule.after);
            before.is_none() || after.is_none() || before.unwrap() < after.unwrap()
        })
    }

    fn get_center_page(pages: &PagesToProduce) -> i32 {
        pages.get(pages.len() / 2).unwrap().clone()
    }

    fn resolve_star1(content: String) -> i32 {
        let (orderings, pages_to_produce_list) = parse(content);
        pages_to_produce_list
            .iter()
            .filter(|pages| check_ordering(&pages, &orderings))
            .map(|pages| get_center_page(&pages))
            .sum()
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn has_bad_ordering(pages: &PagesToProduce, rules: &PageOrderingRules) -> bool {
        rules
            .iter()
            .find(|rule| {
                let before = pages.iter().position(|&page| page == rule.before);
                let after = pages.iter().position(|&page| page == rule.after);
                before.is_some() && after.is_some() && before.unwrap() > after.unwrap()
            })
            .is_some()
    }
    
    fn fix_ordering(pages: &PagesToProduce, rules: &PageOrderingRules) -> PagesToProduce {
        let fixed = rules.iter().fold(pages.clone(), |mut pages, rule| {
            let before = pages.iter().position(|&page| page == rule.before);
            let after = pages.iter().position(|&page| page == rule.after);
            if before.is_some() && after.is_some() && before.unwrap() > after.unwrap() {
                let before = before.unwrap();
                let after = after.unwrap();
                pages.swap(before, after);
                pages
            } else { 
                pages
            }
        });
        
        if fixed == *pages {
            fixed
        } else {
            fix_ordering(&fixed, &rules)
        }
    }

    fn resolve_star2(content: String) -> i32 {
        let (orderings, pages_to_produce_list) = parse(content);
        pages_to_produce_list
            .iter()
            .filter(|pages| has_bad_ordering(&pages, &orderings))
            .map(|pages| fix_ordering(&pages, &orderings))
            .map(|pages| get_center_page(&pages))
            .sum()
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
            let data1 = read_content("data/day05/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 143);

            let data2 = read_content("data/day05/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 5732);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day05/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 123);

            let data2 = read_content("data/day05/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 4716);
        }
    }
}
