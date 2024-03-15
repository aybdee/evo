use crate::graphics::Renderer;
use crate::types::CellIndex;
use core::fmt;
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;

macro_rules! to_cellindex {
    ($position:ident) => {
        ($position.x() as usize, $position.y() as usize)
    };
}

macro_rules! to_point {
    ($x:ident,$y:ident) => {
        ($position.x() as usize, $position.y() as usize)
    };
}

#[derive(Debug)]
pub struct Organism {
    pub position: CellIndex,
}

impl Organism {
    fn new(position: CellIndex) -> Self {
        Organism { position }
    }
}

#[derive(Debug)]
pub struct State<'a> {
    pub grid: Grid,
    pub organisms: HashMap<u32, Organism>,
    //allow intialization without renderer for debugging
    pub renderer: Option<&'a mut Renderer>,
    pub ids: Vec<u32>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        return Self {
            grid: Grid::new(0, 0),
            organisms: HashMap::default(),
            renderer: None,
            ids: vec![],
        };
    }
}

impl<'a> State<'a> {
    pub fn new(row_size: usize, column_size: usize, renderer: Option<&'a mut Renderer>) -> Self {
        match renderer {
            Some(renderer) => {
                if renderer.get_size()
                    == (
                        (column_size as u32) * renderer.point_size,
                        (row_size as u32) * renderer.point_size,
                    )
                {
                    return Self {
                        grid: Grid::new(row_size, column_size),
                        organisms: HashMap::default(),
                        renderer: Some(renderer),
                        ids: vec![],
                    };
                } else {
                    return Self {
                        grid: Grid::new(row_size, column_size),
                        ..Default::default()
                    };
                }
            }
            None => {
                return Self {
                    grid: Grid::new(row_size, column_size),
                    ..Default::default()
                }
            }
        };
    }

    pub fn assign_id(&mut self) -> u32 {
        //just count upwards for now
        //start from 1
        let next_id = self.ids.last().unwrap_or(&(1 as u32)) + 1;
        self.ids.push(next_id);
        return next_id;
    }

    pub fn move_organism(&mut self, id: u32, position: CellIndex) -> Result<(), String> {
        match self.organisms.get_mut(&id) {
            Some(organism) => {
                self.grid.set_cell(position, id);
                organism.position = position;
                Ok(())
            }
            None => Err(String::from("id does not exist")),
        }
    }

    pub fn initialize_organism(&mut self, position: CellIndex) {
        let (x, y) = position;
        let id = self.assign_id();
        let organism = Organism::new((x, y));
        self.organisms.insert(id, organism);
        self.grid.set_cell((x, y), id)
    }

    pub fn initialize_organisms_random(&mut self, num_organisms: usize) {
        self.grid.reset();
        let mut rng = rand::thread_rng();
        let mut organism_position_set: Vec<usize> =
            (0..(self.grid.num_rows * self.grid.num_cols)).collect();
        organism_position_set.shuffle(&mut rng);
        for index in organism_position_set[..num_organisms].iter() {
            let position = (index % self.grid.num_rows, index / self.grid.num_rows);
            self.initialize_organism(position);
        }
    }

    pub fn num_organisms(&self) -> usize {
        return self.grid.num_set();
    }

    pub fn display(&mut self) -> Result<(), String> {
        //check to see if renderer is attached
        match &mut self.renderer {
            Some(renderer) => {
                renderer.clear();
                for organism_index in self.organisms.iter() {
                    let (x, y) = organism_index.1.position;
                    println!("{} {}", x, y);
                    renderer.draw_dot(Point::new(x as i32, y as i32), Color::RED)?;
                }
                renderer.present();
                Ok(())
            }
            None => Err(String::from("renderer not attached")),
        }
    }
}

impl<'a> fmt::Display for State<'a> {
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
struct Cell {
    value: u32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_init() {
        for i in 1..10 {
            let mut state = State::new(i, i, None);
            state.initialize_organisms_random((i * i) - 1);
            println!("{}", state);
            assert!(state.grid.shape() == (i, i));
            assert!(state.num_organisms() == i * i - 1);
        }
    }

    #[test]
    fn test_random_init_rect() {
        let mut state = State::new(200, 500, None);
        state.initialize_organisms_random(200);
        assert!(state.grid.shape() == (200, 500));
        assert!(state.num_organisms() == 200);
    }
}
