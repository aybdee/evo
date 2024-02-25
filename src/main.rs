mod types;
use core::fmt;
use rand::prelude::*;
use types::CellIndex;

#[derive(Debug)]
struct Organism {
    position: CellIndex,
}

impl Organism {
    fn new(position: CellIndex) -> Self {
        Organism { position }
    }
}

#[derive(Debug)]
struct State {
    grid: Grid,
    organisms: Vec<Organism>,
}

impl State {
    fn new(row_size: usize, column_size: usize) -> Self {
        return Self {
            grid: Grid::new(row_size, column_size),
            organisms: vec![],
        };
    }
    fn set_active_update(&mut self, cells: Vec<CellIndex>) {
        self.grid.reset();
        for cell in cells.into_iter() {
            self.grid.set_cell_active(cell)
        }
    }

    fn set_inactive_update(&mut self, cells: Vec<CellIndex>) {
        self.grid.reset();
        for cell in cells.into_iter() {
            self.grid.set_cell_inactive(cell)
        }
    }

    fn initialize_organisms_random(&mut self, num_organisms: usize) {
        let mut rng = rand::thread_rng();
        let mut organism_position_set: Vec<usize> =
            (0..(self.grid.num_rows * self.grid.num_cols)).collect();
        organism_position_set.shuffle(&mut rng);
        let organism_positions: Vec<(usize, usize)> = organism_position_set[..num_organisms]
            .iter()
            .map(|position| {
                let (x, y) = (position % self.grid.num_cols, position / self.grid.num_cols);
                let organisms = Organism::new((x, y));
                self.organisms.push(organisms);
                (x, y)
            })
            .collect();

        self.set_active_update(organism_positions);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = &self.grid;
        write!(f, "grid:\n{grid}")
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    num_rows: usize,
    num_cols: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid_display: String = self
            .cells
            .iter()
            .map(|x| {
                let row: String = x
                    .iter()
                    .map(|cell| (cell.active as u8).to_string())
                    .collect();
                format!("{row} \n")
            })
            .collect();
        write!(f, "{grid_display}")
    }
}

impl Grid {
    fn new(row_size: usize, column_size: usize) -> Self {
        return Self {
            cells: vec![vec![Cell::default(); column_size]; row_size],
            num_rows: row_size,
            num_cols: column_size,
        };
    }

    fn reset(&mut self) {
        self.cells = vec![vec![Cell::default(); self.cells[0].len()]; self.cells.len()]
    }
    fn toggle_cell(&mut self, index: CellIndex) {
        self.cells[index.0][index.1].active = !(self.cells[index.0][index.1].active);
    }

    fn set_cell_active(&mut self, index: CellIndex) {
        self.cells[index.0][index.1].active = true;
    }

    fn set_cell_inactive(&mut self, index: CellIndex) {
        self.cells[index.0][index.1].active = false;
    }
}

#[derive(Clone, Debug)]
struct Cell {
    active: bool,
}
impl Default for Cell {
    fn default() -> Self {
        Cell { active: false }
    }
}

impl Cell {
    fn new(active: bool) -> Self {
        Cell { active }
    }
    fn toggle(&mut self) {
        self.active = !(self.active);
    }

    fn set_active(&mut self) {
        self.active = true;
    }

    fn set_inactive(&mut self) {
        self.active = false;
    }
}

fn main() {
    println!("this is my lil natural selection simulation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_initialized() {
        let mut state = State::new(20, 20);
        state.set_active_update(vec![(0, 1), (0, 2), (0, 3)]);
        println!("{}", state);
    }

    #[test]
    fn test_random_init() {
        for i in 0..10 {
            let mut state = State::new(20, 20);
            state.initialize_organisms_random(200);
            println!("{}", state);
        }
    }
}
