#[derive(PartialEq)]
#[derive(Debug)]
pub struct Formula {
    formula: String,
    world: usize,
    state: FormulaState,
}

impl Formula {
    pub fn new(formula: String, world: usize) -> Self {
        Formula {
            formula,
            world,
            state: FormulaState::Active
        }
    }

    pub fn state(&self) -> &FormulaState {
        &self.state
    }

    pub fn world(&self) -> usize {
        self.world
    }

    pub fn formula(&self) -> &String {
        &self.formula
    }

    pub fn deactivate(&mut self) {
        self.state = FormulaState::Inactive;
    }

    pub fn wait(&mut self) {
        self.state = FormulaState::WaitingNewWorlds;
    }

    pub fn close(&mut self) {
        self.state = FormulaState::Closed;
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum FormulaState {
    Active,
    Inactive,
    WaitingNewWorlds,
    Closed,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct World {
    id: usize,
    c: Option<String>
}

impl World {
    pub fn new(id: usize) -> World {
        World{
            id,
            c: None
        }
    }
}
