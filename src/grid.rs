use core::fmt;

pub type CellIndex = (usize, usize);

pub enum SeekOrientation {
    PosX,
    PosY,
    NegX,
    NegY,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid_display: String = self
            .cells
            .iter()
            .map(|x| {
                let row: String = x
                    .iter()
                    .map(|cell| (cell.value).to_string() + " ")
                    .collect();
                format!("{row} \n")
            })
            .collect();
        write!(f, "{grid_display}")
    }
}

impl Grid {
    pub fn new(row_size: usize, column_size: usize) -> Self {
        return Self {
            cells: vec![vec![Cell::default(); column_size]; row_size],
            num_rows: row_size,
            num_cols: column_size,
        };
    }

    pub fn reset(&mut self) {
        self.cells = vec![vec![Cell::default(); self.cells[0].len()]; self.cells.len()]
    }

    pub fn get(&self, index: CellIndex) -> Cell {
        return self.cells[index.0][index.1].clone();
    }

    pub fn set_cell(&mut self, index: CellIndex, id: u32) {
        self.cells[index.0][index.1].set(id);
    }

    pub fn unset_cell(&mut self, index: CellIndex) {
        self.cells[index.0][index.1].unset();
    }

    pub fn set_cells(&mut self, updates: Vec<(CellIndex, u32)>) {
        for (cell, id) in updates.into_iter() {
            self.set_cell(cell, id)
        }
    }

    pub fn unset_cells(&mut self, cells: Vec<CellIndex>) {
        for cell in cells.into_iter() {
            self.unset_cell(cell)
        }
    }

    pub fn seek_zero(&self, pos: CellIndex, orientation: SeekOrientation) -> bool {
        let (x, y) = pos;
        let mut zero_vec: Vec<u32> = vec![];
        match orientation {
            SeekOrientation::PosX => {
                for i in x + 1..self.num_rows {
                    if self.get((i, y)).value == 0 {
                        return true;
                    }
                }
            }
            SeekOrientation::NegX => {
                for i in (0..x).rev() {
                    if self.get((i, y)).value == 0 {
                        return true;
                    }
                }
            }
            SeekOrientation::PosY => {
                for i in y + 1..(self.num_cols) {
                    if self.get((x, i)).value == 0 {
                        return true;
                    }
                }
            }

            SeekOrientation::NegY => {
                for i in (0..y).rev() {
                    if self.get((x, i)).value == 0 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn shape(&self) -> (usize, usize) {
        //use direct vector length
        if self.cells.len() != 0 {
            return (self.cells.len(), self.cells[0].len());
        } else {
            return (0, 0);
        }
    }

    pub fn is_set(&self, index: CellIndex) -> bool {
        if (self.cells[index.0][index.1].value == 0) {
            return false;
        } else {
            return true;
        }
    }

    pub fn num_set(&self) -> usize {
        let mut num_set_cells = 0;
        for row in self.cells.iter() {
            for cell in row {
                if cell.value != 0 {
                    num_set_cells += 1;
                }
            }
        }
        return num_set_cells;
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub value: u32,
}
impl Default for Cell {
    fn default() -> Self {
        Cell { value: 0 }
    }
}

impl Cell {
    fn new(value: u32) -> Self {
        Cell { value }
    }

    fn set(&mut self, id: u32) {
        self.value = id;
    }

    fn unset(&mut self) {
        self.value = 0;
    }
}
