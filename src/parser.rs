enum ParseState {
    Open,
    Parentheses
}

pub fn parse_formula(formula: &str) -> Result<Instructions, String> {
    let mut int_formula = formula;
    let mut state = ParseState::Open;
    let mut temp_var = String::new();

    let mut variables: Vec<String> = Vec::new();
    let mut operators = String::new();
    // let as_bytes = formula.as_bytes();
    let ops: [char; 6] = ['⊃', '¬', '⋀', '⋁', '◻', '◇'];

    if formula.starts_with("(") && formula.ends_with(")") {
        let strlen = formula.len();
        int_formula = &int_formula[1..strlen];
    }

    for ch in int_formula.chars() {
        match state {
            ParseState::Open => {
                if ch == ' ' { continue; }
                else if ch == '(' { state = ParseState::Parentheses; }
                else if ops.contains(&ch) { operators.push(ch); }
                else if ch.is_alphabetic() { variables.push(ch.to_string()); }
                else { 
                    let err_msg = format!("Unable to parse char {} in {}", ch, formula);
                    return Err(err_msg); 
                }
            },
            ParseState::Parentheses => {
                if ch == ')' { 
                    variables.push(temp_var);
                    temp_var = String::new();
                    state = ParseState::Open; 
                }
                else { temp_var.push(ch); }
            }
        }
    }
    Ok(Instructions { operators, variables })
    }

#[derive(Debug, PartialEq)]
pub struct Instructions {
    operators: String,
    variables: Vec<String>
}

impl Instructions {
    pub fn operators(&self) -> &String {
        &self.operators
    }

    pub fn variables(&self) -> &Vec<String> {
        &self.variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_build() {
        let formula = "p ⊃ q";
        let manual_instructions = Instructions {
            operators: "⊃".to_string(),
            variables: vec!['p'.to_string(), 'q'.to_string()]
        };
    assert_eq!(manual_instructions, parse_formula(formula).unwrap())
    }

    #[test]
    fn parentheses() {
        let formula = "(p ⋀ r) ⊃ q";
        let manual_instructions = Instructions {
            operators: "⊃".to_string(),
            variables: vec!["p ⋀ r".to_string(), "q".to_string()]
        };
    assert_eq!(manual_instructions, parse_formula(formula).unwrap())        
    }
}
