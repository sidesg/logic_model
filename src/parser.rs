pub struct Parser {

}

impl Parser {
    pub fn parse_formula(formula: &str) -> Option<Instructions> {
        // read formula

        // create instructions
        todo!()
    }
}

pub enum InstructionOperator {
    And,
    Or,
    MaterialImplecation,
    Not,
    Necessary,
    Possible
}


pub struct Instructions {
    pub operator: InstructionOperator,
    pub variables: Vec<String>
}
