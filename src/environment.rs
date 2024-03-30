use crate::graphics::Renderer;
use crate::grid::{CellIndex, Grid, SeekOrientation};
use core::fmt;
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Organism {
    pub position: CellIndex,
    pub color: Color,
}
#[derive(Clone)]
pub enum MoveAction {
    Forward,
    Backward,
    Up,
    Down,
}

#[derive(Clone)]
pub enum Action {
    Move(MoveAction),
    InPlace,
}

impl Organism {
    fn new(position: CellIndex) -> Self {
        let mut rng = rand::thread_rng();
        Organism {
            position,
            color: Color {
                r: rng.gen_range(0..=255),
                g: rng.gen_range(0..=255),
                b: rng.gen_range(0..=255),
                a: 255,
            },
        }
    }
    fn get_action(&self) -> Action {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..=4);
        return Action::Move(MoveAction::Forward);
        // if random_number == 0 {
        //     return Action::Move(MoveAction::Down);
        // } else if random_number == 1 {
        //     return Action::Move(MoveAction::Up);
        // } else if random_number == 2 {
        //     return Action::Move(MoveAction::Forward);
        // } else {
        //     return Action::Move(MoveAction::Backward);
        // }
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
                        (column_size as u32) * (renderer.point_size),
                        (row_size as u32) * (renderer.point_size),
                    )
                {
                    return Self {
                        grid: Grid::new(column_size, row_size),
                        organisms: HashMap::default(),
                        renderer: Some(renderer),
                        ids: vec![],
                    };
                } else {
                    return Self {
                        grid: Grid::new(column_size, row_size),
                        ..Default::default()
                    };
                }
            }
            None => {
                return Self {
                    grid: Grid::new(column_size, row_size),
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

    pub fn change_organism_position(&mut self, id: u32, position: CellIndex) -> Result<(), String> {
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
        let mut rng = rand::thread_rng();
        let mut organism_position_set: Vec<usize> =
            (0..(self.grid.num_rows * self.grid.num_cols)).collect();
        organism_position_set.shuffle(&mut rng);
        for index in organism_position_set[..num_organisms].iter() {
            let position = (index % self.grid.num_rows, (index / self.grid.num_rows));
            self.initialize_organism(position);
        }
    }

    pub fn num_organisms(&self) -> usize {
        return self.grid.num_set();
    }

    pub fn display(&mut self) -> Result<(), String> {
        //check to see if renderer is attached
        let mut rng = rand::thread_rng();
        match &mut self.renderer {
            Some(renderer) => {
                renderer.clear();
                for organism_entry in self.organisms.iter() {
                    let (x, y) = organism_entry.1.position;
                    renderer.draw_dot(Point::new(x as i32, y as i32), organism_entry.1.color)?;
                }
                renderer.present();
                Ok(())
            }
            None => Err(String::from("renderer not attached")),
        }
    }

    pub fn clear_display(&mut self) -> Result<(), String> {
        //check to see if renderer is attached
        match &mut self.renderer {
            Some(renderer) => {
                renderer.clear();
                Ok(())
            }
            None => Err(String::from("renderer not attached")),
        }
    }

    pub fn handle_move(&mut self, organism_entry: (u32, Organism), motion: MoveAction) {
        let (id, organism) = organism_entry;
        match motion {
            MoveAction::Forward => {
                let (x, y) = organism.position;
                self.change_organism_position(id, (x + 1, y)).unwrap();
            }
            MoveAction::Backward => {
                let (x, y) = organism.position;
                self.change_organism_position(id, (x - 1, y)).unwrap();
            }
            MoveAction::Down => {
                let (x, y) = organism.position;
                self.change_organism_position(id, (x, y + 1)).unwrap();
            }
            MoveAction::Up => {
                let (x, y) = organism.position;
                self.change_organism_position(id, (x, y - 1)).unwrap();
            }
        }
    }

    pub fn handle_inplace(&mut self, organism_entry: (u32, Organism)) {
        let (id, organism) = organism_entry;
        //keep same position
        //necessary because grid resets
        self.change_organism_position(id, organism.position)
            .unwrap();
    }

    pub fn handle_action(&mut self, organism_entry: (u32, Organism), action: Action) {
        match action {
            Action::Move(motion) => self.handle_move(organism_entry, motion),
            Action::InPlace => self.handle_inplace(organism_entry),
        }
    }

    pub fn validate_move(&self, organism_entry: (u32, Organism), motion: MoveAction) -> bool {
        let (id, organism) = organism_entry;
        match motion {
            MoveAction::Forward => {
                let (x, y) = organism.position;
                if x < self.grid.shape().0 - 1
                    && self
                        .grid
                        .seek_zero(organism.position, SeekOrientation::PosX)
                {
                    return true;
                }
            }
            MoveAction::Backward => {
                let (x, y) = organism.position;
                if x > 0
                    && self
                        .grid
                        .seek_zero(organism.position, SeekOrientation::NegX)
                {
                    return true;
                }
            }

            MoveAction::Up => {
                let (x, y) = organism.position;
                if y > 0
                    && self
                        .grid
                        .seek_zero(organism.position, SeekOrientation::NegY)
                {
                    return true;
                }
            }

            MoveAction::Down => {
                let (x, y) = organism.position;
                if y < self.grid.shape().1 - 1
                    && self
                        .grid
                        .seek_zero(organism.position, SeekOrientation::PosY)
                {
                    // self.change_organism_position(id, (x, y + 1)).unwrap();
                    return true;
                }
            }
        }
        return false;
    }

    pub fn validate_action(&self, organism_entry: (u32, Organism), action: Action) -> bool {
        match action {
            Action::Move(motion) => return self.validate_move(organism_entry, motion),
            Action::InPlace => {
                return true;
            }
        }
    }

    pub fn step(&mut self) {
        let valid_action_pairs: Vec<((u32, Organism), Action)> = self
            .organisms
            .clone()
            .into_iter()
            .map(|organism_entry| (organism_entry.clone(), organism_entry.1.get_action()))
            .map(|action_pair| {
                if self.validate_action(action_pair.0.clone(), action_pair.1.clone()) {
                    action_pair
                } else {
                    (action_pair.0.clone(), Action::InPlace)
                }
            })
            .collect();

        self.grid.reset();
        for (organism_entry, action) in valid_action_pairs.into_iter() {
            self.handle_action(organism_entry, action);
        }
    }
}

impl<'a> fmt::Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = &self.grid;
        write!(f, "grid:\n{grid}")
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
