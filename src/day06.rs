mod day06 {
    use im::{hashmap, HashMap};
    use std::str::Chars;
    use tailcall::tailcall;

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
    }

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    struct Cell {
        content: char,
        obstacle: bool,
    }
    impl Cell {
        fn new(content: char, obstacle: bool) -> Cell {
            Cell { content, obstacle }
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    enum Mover {
        Up,
        Down,
        Left,
        Right,
    }

    impl Mover {
        fn TURNS(from: &Mover) -> Mover {
            match from {
                Mover::Up => Mover::Right,
                Mover::Right => Mover::Down,
                Mover::Down => Mover::Left,
                Mover::Left => Mover::Up,
            }
        }
        fn MAPPINGS(ch: char) -> Mover {
            match ch {
                '^' => Mover::Up,
                'v' => Mover::Down,
                '<' => Mover::Left,
                '>' => Mover::Right,
                _ => panic!("Unknown mover"),
            }
        }

        fn go(coord: &Coord, mover: &Mover) -> Coord {
            match mover {
                Mover::Up => coord.up(),
                Mover::Right => coord.right(),
                Mover::Down => coord.down(),
                Mover::Left => coord.left(),
            }
        }
    }


    // ------------------------------------------------------------------------------

    fn parse(input: String) -> im::HashMap<Coord, Cell> {
        im::HashMap::from_iter(input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, content)| {
                (
                    Coord::new(x as i32, y as i32),
                    Cell::new(content, content == '#'),
                )
            })
        }))
    }

    fn find_start(area: &im::HashMap<Coord, Cell>) -> Option<(Coord, Mover)> {
        for (coord, cell) in area.iter() {
            if "<>v^".contains(cell.content) {
                return Some((coord.clone(), Mover::MAPPINGS(cell.content).clone()));
            }
        }
        None
    }

    // -----------------------------------------------------------------------------------------------------------------

    fn worker1(
        area: &HashMap<Coord, Cell>,
        start_coord: &Coord,
        start_mover: &Mover,
        current: &Coord,
        mover: &Mover,
        visited: im::HashSet<Coord>,
    ) -> im::HashSet<Coord> {
        let next_coord = Mover::go(current, &mover);
        if start_mover == mover && start_coord == current && visited.contains(current) {
            visited
        } else {
            match area.get(&next_coord) {
                None => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(current.clone());
                    new_visited
                }
                Some(cell) if cell.obstacle => worker1(
                    &area,
                    &start_coord,
                    &start_mover,
                    &current,
                    &Mover::TURNS(mover),
                    visited,
                ),
                Some(cell) => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(current.clone());
                    worker1(
                        &area,
                        &start_coord,
                        &start_mover,
                        &next_coord,
                        &mover,
                        new_visited,
                    )
                }
            }
        }
    }

    fn walk1(
        area: &HashMap<Coord, Cell>,
        start_coord: &Coord,
        start_mover: &Mover,
    ) -> im::HashSet<Coord> {
        worker1(
            area,
            start_coord,
            start_mover,
            start_coord,
            start_mover,
            im::HashSet::new(),
        )
    }

    fn resolve_star1(input: String) -> i32 {
        let area = parse(input);
        match find_start(&area) {
            None => -1,
            Some((coord, mover)) => walk1(&area, &coord, &mover).len() as i32,
        }
    }

    // -----------------------------------------------------------------------------------------------------------------

    #[tailcall]
    fn is_loop_worker(
        area: HashMap<Coord, Cell>,
        current: (Coord, Mover),
        visited: im::HashSet<(Coord, Mover)>,
    ) -> bool {
        // println!(
        //     "is_loop_worker: current: {:?} - {}",
        //     current,
        //     visited.len());
        if visited.contains(&current) {
            true
        } else {
            let (coord, mover) = current.clone();
            let next_coord = Mover::go(&coord, &mover);
            match area.get(&next_coord) {
                None => false,
                Some(cell) if cell.obstacle => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(current);
                    is_loop_worker(area, (coord.clone(), Mover::TURNS(&mover)), new_visited)
                }
                Some(cell) => {
                    let mut new_visited = visited.clone();
                    new_visited.insert(current);
                    is_loop_worker(area, (next_coord, mover.clone()), new_visited)
                }
            }
        }
    }

    fn walk2(area: &HashMap<Coord, Cell>, start_coord: Coord, start_mover: Mover) -> i32 {
        let obstacle = Cell::new('O', true);

        area.iter()
            .filter(|(coord, cell)| !cell.obstacle && !start_coord.eq(coord))
            .filter(|(coord, cell)| {
                let mut updated_area = area.clone();
                updated_area.insert((*coord).clone(), obstacle.clone());
                is_loop_worker(
                    updated_area,
                    (start_coord.clone(), start_mover.clone()),
                    im::HashSet::new(),
                )
            })
            .count() as i32
    }

    fn resolve_star2(input: String) -> i32 {
        let area = parse(input);
        match find_start(&area) {
            None => -1,
            Some((coord, mover)) => walk2(&area, coord, mover),
        }
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
            let data1 = read_content("data/day06/example-1.txt");
            let result1 = super::resolve_star1(data1);
            assert_eq!(result1, 41);

            let data2 = read_content("data/day06/puzzle-1.txt");
            let result2 = super::resolve_star1(data2);
            assert_eq!(result2, 4776);
        }

        #[test]
        fn result_star2_test() {
            let data1 = read_content("data/day06/example-1.txt");
            let result1 = super::resolve_star2(data1);
            assert_eq!(result1, 6);

            let data2 = read_content("data/day06/puzzle-1.txt");
            let result2 = super::resolve_star2(data2);
            assert_eq!(result2, 1586);
        }
    }
}
