use std::{collections::HashMap, rc::Rc};

pub struct GameState {
    pub rules: Rules,
    pub board: Board,
    pub pawns: Vec<Pawn>,
    pub schedule: HashMap<i32, Turn>,
    pub turn: i32
}

impl GameState {
    pub fn new(width: i32, height: i32, rules: Rules) -> Self {
        let mut schedule = HashMap::new();
        schedule.insert(0, Self::create_first_turn(&rules));

        GameState {
            rules,
            board: Board::new(width, height),
            pawns: vec![],
            schedule,
            turn: 0
        }
    }

    fn create_first_turn(rules: &Rules) -> Turn {
        let mut setup = Vec::<Rc<dyn Event>>::new();
        for _ in 0..rules.max_pawn_per_player {
            setup.push(Rc::new(EventSpawnPawn {
                owner: Entity::Player1
            }));
            setup.push(Rc::new(EventSpawnPawn {
                owner: Entity::Player2
            }));
        }

        Turn {
            setup,
            playtime: vec![],
            after: vec![]
        }
    }
}

pub struct Game {
    pub state: GameState,
    pub turns_rollbacks: Vec<Vec<Box<Rollback>>>
}

impl Game {
    pub fn new(width: i32, height: i32, rules: Rules) -> Self {
        Game {
            state: GameState::new(width, height, rules),
            turns_rollbacks: vec![]
        }
    }

    fn apply_event(&mut self, event: &Rc<dyn Event>) -> Vec<Box<Rollback>> {
        let mut rollbacks: Vec<Box<Rollback>> = vec![];

        if event.as_ref().is_allowed(&self.state) == false {
            return rollbacks;
        }

        let modifiers = event.as_ref().build_modifiers(&self.state);
        for m in modifiers {
            rollbacks.push(m.apply(&mut self.state))
        }

        rollbacks
    }

    pub fn apply_current_turn(&mut self) {
        let mut rollbacks: Vec<Box<Rollback>> = vec![];
        let schedule = self.state.schedule.clone();
        let turn: &Turn = schedule.get(&self.state.turn).unwrap();
        
        for event in turn.setup.iter().chain(turn.playtime.iter()).chain(turn.after.iter()) {
            rollbacks.append(&mut self.apply_event(event));
        }

        self.state.turn += 1;
        self.turns_rollbacks.push(rollbacks);
    }

    pub fn rollback_last_turn(&mut self) {
        for rollback in self.turns_rollbacks.pop().unwrap().iter().rev() {
            rollback(&mut self.state)
        }
    }
}

pub struct Rules {
    pub max_pawn_per_player: usize
}

#[derive(Clone)]
pub struct Turn {
    pub setup: Vec<Rc<dyn Event>>,
    pub playtime: Vec<Rc<dyn Event>>,
    pub after: Vec<Rc<dyn Event>>,
}

pub trait Event {
    fn is_allowed(&self, state: &GameState) -> bool;
    fn build_modifiers(&self, state: &GameState) -> Vec<Box<dyn Modifier>>;
}

pub trait Modifier {
    fn apply(&self, state: &mut GameState) -> Box<Rollback>;
}

pub type Rollback = dyn Fn(&mut GameState);

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Board { 
            width, 
            height,
            tiles: vec![vec![Tile::new(); width as usize]; height as usize]
        }
    }
}

#[derive(Clone)]
pub enum Tile {
    Empty,
    Occupied(Occupied)
}

impl Tile {
    pub fn new() -> Self {
        Tile::Empty
    }
}

#[derive(Clone)]
pub struct Occupied {
    pub pawn_id: usize
}

#[derive(Debug)]
pub struct Pawn {
    pub id: usize,
    pub owner: Entity,
    pub state: PawnState
}

impl Pawn {
    pub fn new(id: usize) -> Self {
        Pawn { id, owner: Entity::Gaia, state: PawnState::Staging }
    }
}

#[derive(Debug)]
pub enum PawnState {
    Placed(Position),
    Staging,
    Dead
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Entity {
    Player1,
    Player2,
    Gaia
}

#[derive(Clone)]
pub struct EventSpawnPawn {
    pub owner: Entity
}

impl Event for EventSpawnPawn {
    fn is_allowed(&self, state: &GameState) -> bool {
        state.pawns.iter().filter(|p| p.owner == self.owner).count() < state.rules.max_pawn_per_player
    }

    fn build_modifiers(&self, state: &GameState) -> Vec<Box<dyn Modifier>> {
        vec![
            Box::new(ModifierCreatePawn {}),
            Box::new(ModifierChangePawnOwner {
                pawn_id: state.pawns.len(),
                new_owner: self.owner
            })
        ]
    }
}

pub struct ModifierCreatePawn;

impl Modifier for ModifierCreatePawn {
    fn apply(&self, state: &mut GameState) -> Box<Rollback> {
        let index = state.pawns.len();
        state.pawns.push(Pawn::new(index));

        Box::new(move |state| {
            state.pawns.remove(index);
        })
    }
}

pub struct ModifierChangePawnOwner {
    pub pawn_id: usize,
    pub new_owner: Entity
}

impl Modifier for ModifierChangePawnOwner {
    fn apply(&self, state: &mut GameState) -> Box<Rollback> {
        let id = self.pawn_id;
        let mut pawn = state.pawns.get_mut(id).unwrap();
        let previous_owner = pawn.owner;
        pawn.owner = self.new_owner;

        Box::new(move |state| {
            let mut pawn = state.pawns.get_mut(id).unwrap();
            pawn.owner = previous_owner;
        })
    }
}