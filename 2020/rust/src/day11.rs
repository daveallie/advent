#[derive(Clone, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Clone)]
struct Cell {
    is_seat: bool,
    occupied: bool,
    visible_occupied: usize,
}

impl Cell {
    fn new(is_seat: bool, occupied: bool) -> Self {
        Self {
            is_seat,
            occupied,
            visible_occupied: 0,
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let rows = cells.len();
        let cols = cells[0].len();

        Self {
            cells,
            rows,
            cols,
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).map(|r| r.get(col)).flatten()
    }

    fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).map(|r| r.get_mut(col)).flatten()
    }

    fn count_occupied(&self) -> usize {
        self.cells.iter()
            .flat_map(|row| row.iter())
            .filter(|c| c.occupied)
            .count()
    }

    fn calc_visible_occupied_dir(&mut self, row: usize, col: usize, long_range: bool, dir: Dir) -> bool {
        let (adj_row, adj_col) = match dir {
            Dir::Up => (row - 1, col),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
            Dir::Right => (row, col + 1),
            Dir::UpLeft => (row - 1, col - 1),
            Dir::UpRight => (row - 1, col + 1),
            Dir::DownLeft => (row + 1, col - 1),
            Dir::DownRight => (row + 1, col + 1),
        };

        let adj_cell = self.get_cell(adj_row, adj_col);

        let res = match adj_cell {
            Some(adj_c) => {
                if adj_c.is_seat {
                    adj_c.occupied
                } else if !long_range {
                    false
                } else {
                    self.calc_visible_occupied_dir(adj_row, adj_col, true, dir.clone())
                }
            },
            None => false,
        };

        let cell = self.get_cell_mut(row, col).unwrap();
        if cell.is_seat && res {
            cell.visible_occupied += 1;
        }

        res
    }

    fn calc_visible_occupied(&mut self, row: usize, col: usize, long_range: bool) {
        self.calc_visible_occupied_dir(row, col, long_range, Dir::Up);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::Down);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::Left);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::Right);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::UpLeft);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::UpRight);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::DownLeft);
        self.calc_visible_occupied_dir(row, col, long_range, Dir::DownRight);
    }

    fn update(&mut self, part2: bool) -> bool {
        let occ_count = self.count_occupied();

        self.cells.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|cell| {
                if !cell.is_seat {
                    return
                }

                match cell.visible_occupied {
                    0 if !cell.occupied => cell.occupied = true,
                    4..=8 if cell.occupied && !part2 => cell.occupied = false,
                    5..=8 if cell.occupied && part2 => cell.occupied = false,
                    _ => (),
                }

                cell.visible_occupied = 0;
            });

        self.count_occupied() != occ_count
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Grid {
    let cells = input
        .lines()
        .map(|line| line.chars().map(|c| Cell::new(c != '.', c == '#')).collect::<Vec<Cell>>())
        .collect::<Vec<Vec<Cell>>>();

    Grid::new(cells)
}

#[aoc(day11, part1)]
fn solve_part1(grid: &Grid) -> usize {
    solve(grid, false)
}

#[aoc(day11, part2)]
fn solve_part2(grid: &Grid) -> usize {
    solve(grid, true)
}

fn solve(grid: &Grid, part2: bool) -> usize {
    let mut grid: Grid = grid.clone();

    loop {
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if !grid.cells[row][col].is_seat {
                    continue
                }

                grid.calc_visible_occupied(row, col, part2);
            }
        }

        if !grid.update(part2) {
            break
        }
    }

    grid.count_occupied()
}
