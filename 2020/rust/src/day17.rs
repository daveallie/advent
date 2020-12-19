#[derive(Clone, Eq, Hash, PartialEq)]
enum Dir {
    W(bool),
    X(bool),
    Y(bool),
    Z(bool),
    WX(bool, bool),
    WY(bool, bool),
    WZ(bool, bool),
    XY(bool, bool),
    XZ(bool, bool),
    YZ(bool, bool),
    WXY(bool, bool, bool),
    WXZ(bool, bool, bool),
    WYZ(bool, bool, bool),
    XYZ(bool, bool, bool),
    WXYZ(bool, bool, bool, bool),
}

impl Dir {
    fn all_dirs() -> [Self; 80] {
        [
            Self::X(false), Self::X(true), Self::Y(false), Self::Y(true), Self::Z(false), Self::Z(true),
            Self::W(false), Self::W(true),
            Self::XY(false, false), Self::XY(false, true), Self::XY(true, false), Self::XY(true, true),
            Self::XZ(false, false), Self::XZ(false, true), Self::XZ(true, false), Self::XZ(true, true),
            Self::YZ(false, false), Self::YZ(false, true), Self::YZ(true, false), Self::YZ(true, true),
            Self::WX(false, false), Self::WX(false, true), Self::WX(true, false), Self::WX(true, true),
            Self::WY(false, false), Self::WY(false, true), Self::WY(true, false), Self::WY(true, true),
            Self::WZ(false, false), Self::WZ(false, true), Self::WZ(true, false), Self::WZ(true, true),
            Self::XYZ(false, false, false), Self::XYZ(false, false, true), Self::XYZ(false, true, false),
            Self::XYZ(false, true, true), Self::XYZ(true, false, false), Self::XYZ(true, false, true),
            Self::XYZ(true, true, false), Self::XYZ(true, true, true),
            Self::WXY(false, false, false), Self::WXY(false, false, true), Self::WXY(false, true, false),
            Self::WXY(false, true, true), Self::WXY(true, false, false), Self::WXY(true, false, true),
            Self::WXY(true, true, false), Self::WXY(true, true, true),
            Self::WXZ(false, false, false), Self::WXZ(false, false, true), Self::WXZ(false, true, false),
            Self::WXZ(false, true, true), Self::WXZ(true, false, false), Self::WXZ(true, false, true),
            Self::WXZ(true, true, false), Self::WXZ(true, true, true),
            Self::WYZ(false, false, false), Self::WYZ(false, false, true), Self::WYZ(false, true, false),
            Self::WYZ(false, true, true), Self::WYZ(true, false, false), Self::WYZ(true, false, true),
            Self::WYZ(true, true, false), Self::WYZ(true, true, true),
            Self::WXYZ(false, false, false, false), Self::WXYZ(false, false, false, true),
            Self::WXYZ(false, false, true, false), Self::WXYZ(false, false, true, true),
            Self::WXYZ(false, true, false, false), Self::WXYZ(false, true, false, true),
            Self::WXYZ(false, true, true, false), Self::WXYZ(false, true, true, true),
            Self::WXYZ(true, false, false, false), Self::WXYZ(true, false, false, true),
            Self::WXYZ(true, false, true, false), Self::WXYZ(true, false, true, true),
            Self::WXYZ(true, true, false, false), Self::WXYZ(true, true, false, true),
            Self::WXYZ(true, true, true, false), Self::WXYZ(true, true, true, true),
        ]
    }
}

#[derive(Clone)]
struct Cell {
    is_on: bool,
    visible_on: usize,
}

impl Cell {
    fn new(is_on: bool) -> Self {
        Self {
            is_on,
            visible_on: 0,
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Vec<Vec<Cell>>>>,
    w_size: usize,
    x_size: usize,
    y_size: usize,
    z_size: usize,
}

impl Grid {
    fn new(start_enabled: Vec<Vec<bool>>, w_enabled: bool) -> Self {
        let w_size = if w_enabled { 20 } else { 1 };
        let x_size = 20;
        let y_size = 20;
        let z_size = 20;

        let mut cells = (0..w_size).map(|_|
            (0..x_size).map(|_|
                (0..y_size).map(|_|
                    (0..z_size).map(|_| Cell::new(false)).collect::<Vec<Cell>>()
                ).collect::<Vec<Vec<Cell>>>()
            ).collect::<Vec<Vec<Vec<Cell>>>>()
        ).collect::<Vec<Vec<Vec<Vec<Cell>>>>>();


        let w_start = w_size / 2;
        let x_start = (x_size - start_enabled.len()) / 2;
        let y_start = (y_size - start_enabled[0].len()) / 2;

        start_enabled.into_iter().enumerate().for_each(|(x_mod, row)| {
            row.into_iter().enumerate().for_each(|(y_mod, is_on)| {
                if !is_on {
                    return
                }

                let cube: &mut Vec<Vec<Vec<Cell>>> = cells.get_mut(w_start).unwrap();
                let plane: &mut Vec<Vec<Cell>> = cube.get_mut(x_start + x_mod).unwrap();
                let line: &mut Vec<Cell> = plane.get_mut(y_start + y_mod).unwrap();
                let cell = line.get_mut(10).unwrap();
                cell.is_on = true;
            });
        });

        Self {
            cells,
            w_size,
            x_size,
            y_size,
            z_size,
        }
    }

    fn get_cell(&self, w: usize, x: usize, y: usize, z: usize) -> Option<&Cell> {
        self.cells.get(w)
            .map(|cube| cube.get(x)).flatten()
            .map(|plane| plane.get(y)).flatten()
            .map(|row| row.get(z)).flatten()
    }

    fn get_cell_mut(&mut self, w: usize, x: usize, y: usize, z: usize) -> Option<&mut Cell> {
        self.cells.get_mut(w)
            .map(|cube| cube.get_mut(x)).flatten()
            .map(|plane| plane.get_mut(y)).flatten()
            .map(|row| row.get_mut(z)).flatten()
    }

    fn count_on(&self) -> usize {
        self.cells.iter()
            .flat_map(|cube| cube.iter())
            .flat_map(|plane| plane.iter())
            .flat_map(|row| row.iter())
            .filter(|c| c.is_on)
            .count()
    }

    fn calc_visible_on_dir(&mut self, w: usize, x: usize, y: usize, z: usize, dir: &Dir) -> bool {
        let (adj_w, adj_x, adj_y, adj_z) = match dir {
            Dir::W(wb) => (w + 2 * usize::from(*wb) - 1, x, y, z),
            Dir::X(xb) => (w, x + 2 * usize::from(*xb) - 1, y, z),
            Dir::Y(yb) => (w, x, y + 2 * usize::from(*yb) - 1, z),
            Dir::Z(zb) => (w, x, y, z + 2 * usize::from(*zb) - 1),
            Dir::WX(wb, xb) => (w + 2 * usize::from(*wb) - 1, x + 2 * usize::from(*xb) - 1, y, z),
            Dir::WY(wb, yb) => (w + 2 * usize::from(*wb) - 1, x, y + 2 * usize::from(*yb) - 1, z),
            Dir::WZ(wb, zb) => (w + 2 * usize::from(*wb) - 1, x, y, z + 2 * usize::from(*zb) - 1),
            Dir::XY(xb, yb) => (w, x + 2 * usize::from(*xb) - 1, y + 2 * usize::from(*yb) - 1, z),
            Dir::XZ(xb, zb) => (w, x + 2 * usize::from(*xb) - 1, y, z + 2 * usize::from(*zb) - 1),
            Dir::YZ(yb, zb) => (w, x, y + 2 * usize::from(*yb) - 1, z + 2 * usize::from(*zb) - 1),
            Dir::WXY(wb, xb, yb) => (w + 2 * usize::from(*wb) - 1, x + 2 * usize::from(*xb) - 1, y + 2 * usize::from(*yb) - 1, z),
            Dir::WXZ(wb, xb, zb) => (w + 2 * usize::from(*wb) - 1, x + 2 * usize::from(*xb) - 1, y, z + 2 * usize::from(*zb) - 1),
            Dir::WYZ(wb, yb, zb) => (w + 2 * usize::from(*wb) - 1, x, y + 2 * usize::from(*yb) - 1, z + 2 * usize::from(*zb) - 1),
            Dir::XYZ(xb, yb, zb) => (w, x + 2 * usize::from(*xb) - 1, y + 2 * usize::from(*yb) - 1, z + 2 * usize::from(*zb) - 1),
            Dir::WXYZ(wb, xb, yb, zb) => (w + 2 * usize::from(*wb) - 1, x + 2 * usize::from(*xb) - 1, y + 2 * usize::from(*yb) - 1, z + 2 * usize::from(*zb) - 1),
        };

        let adj_cell = self.get_cell(adj_w, adj_x, adj_y, adj_z);

        let res = match adj_cell {
            Some(adj_c) => adj_c.is_on,
            None => false,
        };

        if res {
            let cell = self.get_cell_mut(w, x, y, z).unwrap();
            cell.visible_on += 1;
        }

        res
    }

    fn calc_visible_on(&mut self, w: usize, x: usize, y: usize, z: usize) {
        for dir in Dir::all_dirs().iter() {
            self.calc_visible_on_dir(w, x, y, z, dir);
        }
    }

    fn update(&mut self) -> bool {
        let on_count = self.count_on();

        self.cells.iter_mut()
            .flat_map(|cube| cube.iter_mut())
            .flat_map(|plane| plane.iter_mut())
            .flat_map(|row| row.iter_mut())
            .for_each(|cell| {
                if cell.is_on {
                    match cell.visible_on {
                        2..=3 => (),
                        _ => cell.is_on = false,
                    }
                } else {
                    if cell.visible_on == 3 {
                        cell.is_on = true;
                    }
                }

                cell.visible_on = 0;
            });

        self.count_on() != on_count
    }
}

fn build_grid(input: &str, part2: bool) -> Grid {
    let start_enabled = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    Grid::new(start_enabled, part2)
}

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, false)
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
    let mut grid = build_grid(input, part2);

    for _ in 0..6 {
        for w in 0..grid.w_size {
            for x in 0..grid.x_size {
                for y in 0..grid.y_size {
                    for z in 0..grid.z_size {
                        grid.calc_visible_on(w, x, y, z);
                    }
                }
            }
        }
        grid.update();
    }

    grid.count_on()
}
