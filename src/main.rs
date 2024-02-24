mod types;
use core::fmt;
use types::CellIndex;

#[derive(Debug)]
struct State {
    grid: Grid,
}

impl State {
    fn new(row_size: usize, column_size: usize) -> Self {
        return Self {
            grid: Grid::new(row_size, column_size),
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
}
