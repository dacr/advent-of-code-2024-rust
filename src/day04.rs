mod day04 {
    use std::collections::HashMap;
    use std::str::Chars;

    #[derive(Eq, PartialEq, Hash, Debug, Clone)]
    struct Coord {
        x: i32,
        y: i32,
    }

    impl Coord {
        fn new(x: i32, y: i32) -> Coord {
            Coord { x, y }
        }
        fn up(&self) -> Coord {
            Coord::new(self.x, self.y - 1)
        }
        fn down(&self) -> Coord {
            Coord::new(self.x, self.y + 1)
        }
        fn left(&self) -> Coord {
            Coord::new(self.x - 1, self.y)
        }
        fn right(&self) -> Coord {
            Coord::new(self.x + 1, self.y)
        }
        fn left_up(&self) -> Coord {
            Coord::new(self.x - 1, self.y - 1)
        }
        fn left_down(&self) -> Coord {
            Coord::new(self.x - 1, self.y + 1)
        }
        fn right_up(&self) -> Coord {
            Coord::new(self.x + 1, self.y - 1)
        }
        fn right_down(&self) -> Coord {
            Coord::new(self.x + 1, self.y + 1)
        }
    }

    #[derive(Debug)]
    struct Cell {
        letter: char,
    }
    impl Cell {
        fn new(letter: char) -> Cell {
            Cell { letter }
        }
    }

    fn moves() -> Vec<Box<dyn Fn(&Coord) -> Coord>> {
        vec![
            Box::new(Coord::up),
            Box::new(Coord::down),
            Box::new(Coord::left),
            Box::new(Coord::right),
            Box::new(Coord::left_up),
            Box::new(Coord::left_down),
            Box::new(Coord::right_up),
            Box::new(Coord::right_down),
        ]
    }
    // ------------------------------------------------------------------------------

    fn parse(input: String) -> HashMap<Coord, Cell> {
        let mut matrix = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, letter) in line.chars().enumerate() {
                matrix.insert(Coord::new(x as i32, y as i32), Cell::new(letter));
            }
        }
        matrix
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn check_for(
        mut word: Chars,
        coord: &Coord,
        move_fn: &dyn Fn(&Coord) -> Coord,
        matrix: &HashMap<Coord, Cell>,
    ) -> bool {
        let next_coord = move_fn(&coord);
        match word.next() {
            None => true,
            Some(ch) => match matrix.get(coord) {
                Some(cell) if cell.letter == ch => check_for(word, &next_coord, move_fn, matrix),
                Some(_) => false,
                None => false,
            },
        }
    }

    fn resolve_star1(content: String) -> i32 {
        let matrix = parse(content);
        matrix
            .keys()
            .map(|coord| {
                moves()
                    .into_iter()
                    .filter(|move_fn| check_for("XMAS".chars(), coord, move_fn, &matrix))
                    .count() as i32
            })
            .sum()
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn check_for_x(coord: &Coord, matrix: &HashMap<Coord, Cell>) -> bool {
        vec![
            vec![coord.left_up(), coord.clone(), coord.right_down()],
            vec![coord.right_down(), coord.clone(), coord.left_up()],
            vec![coord.left_down(), coord.clone(), coord.right_up()],
            vec![coord.right_up(), coord.clone(), coord.left_down()],
        ]
        .iter()
        .filter(|v| {
            v.iter()
                .flat_map(|c| matrix.get(c).map(|c| c.letter))
                .collect::<Vec<_>>()
                == vec!['M', 'A', 'S']
        })
        .count()
            == 2
    }

    fn resolve_star2(content: String) -> i32 {
        let matrix = parse(content);
        matrix
            .keys()
            .filter(|coord| check_for_x(coord, &matrix))
            .count() as i32
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
            let data1 = read_content("data/day04/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 18);

            let data2 = read_content("data/day04/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 2551);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day04/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 9);

            let data2 = read_content("data/day04/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 1985);
        }
    }
}
