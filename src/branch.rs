use std::rc::Rc;
use crate::world::World;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Branch {
    formulas: Vec<Node>,
    children: Option<Vec<Box<Branch>>>,
    state: BranchState
}

#[derive(PartialEq)]
#[derive(Debug)]
enum BranchState {
    NonTerminal,
    Open,
    Closed
}

impl Branch {
    pub fn make_root(formulas: Vec<String>, world0: &Rc::<World>) -> Branch {
        Branch {
            formulas: formulas
                .into_iter()
                .map(|exp| Node::new(exp, Rc::clone(world0)))
                .collect(),
            children: None,
            state: BranchState::Open
        }
    }

    // pub fn get_first_active_branch(&mut self) -> Option<&mut Branch> {
    //     if let Some(first_node) = Branch::inner_get_first_active(&mut self.formulas) {
    //         return Some(self)
    //     } else if let Some(children) = &mut self.children {
    //         if let Some(first_active_branch) = children
    //             .iter_mut()
    //             .filter(|child_branch| child_branch.get_first_active_branch().is_some())
    //             .next() {
    //                 Some(first_active_branch)
    //             } else {
    //                 None
    //             }
    //     } else {
    //         None
    //     }      
    // }

    pub fn get_first_active_node(&mut self) -> Option<&mut Node> {
        if let Some(first_node) = Branch::inner_get_first_active(&mut self.formulas) {
            return Some(first_node)
        } else if let Some(children) = &mut self.children {
            let mut first_nodes = children
                .iter_mut()
                .map(|child_branch| child_branch.get_first_active_node());
            first_nodes.next()?
        } else {
            None
        }
    }

    fn inner_get_first_active(formulas: &mut Vec<Node>) -> Option<&mut Node> {
        formulas.iter_mut()
            .filter(|node| node.state == NodeState::Active)
            .next()
    }

    fn add_child(&mut self, branch: Branch) {
        if let Some(children) = &mut self.children {
            children.push(Box::new(branch));
        } else {
            self.children = Some(vec![Box::new(branch)]);
        }
    }

    pub fn get_unclosed(&self) -> Option<Vec<& Branch>> {
        todo!()
    }

    pub fn add_fork(&mut self) {
        todo!()
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum NodeState {
    Active,
    Inactive,
    WaitingNewWorlds,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node {
    pub formula: String,
    world: Rc<World>,
    state: NodeState,
}

impl Node {
    fn new(formula: String, world: Rc<World>) -> Node {
        Node {
            formula: formula,
            world: world,
            state: NodeState::Active
        }
    }

    pub fn deactivate(&mut self) {
        self.state = NodeState::Inactive;
    }

    pub fn wait(&mut self) {
        self.state = NodeState::WaitingNewWorlds;
    }
}

#[test]
fn get_actives() {
    // create tableaux  
    let w0 = Rc::new(World::first_world());
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];

    let mut rootbranch = Branch::make_root(rootformulas, &w0);

    // get first active formula
    let first_active = rootbranch.get_first_active_node().unwrap();

    // assertion
    assert_eq!(first_active.formula, String::from("first formula"));

    first_active.deactivate();

    let first_active = rootbranch.get_first_active_node().unwrap();
    assert_eq!(first_active.formula, String::from("second formula"));
}

#[test]
fn get_actives_recursive() {
    let world0 = Rc::new(World::first_world());
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let mut rootbranch = Branch::make_root(rootformulas, &world0);

    let leaf1formulas: Vec<String> = vec![
        String::from("leaf1 first formula"),
        String::from("leaf1 second formula"),
        String::from("leaf1 third formula")
    ];
    let leaf1 = Branch::make_root(leaf1formulas, &world0);

    let leaf2formulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let leaf2 = Branch::make_root(leaf2formulas, &world0);

    rootbranch.add_child(leaf1);
    rootbranch.add_child(leaf2);


    for formula in rootbranch.formulas.iter_mut() {
        formula.deactivate();
    }

    let first_active = rootbranch.get_first_active_node().unwrap();
    assert_eq!(first_active.formula, String::from("leaf1 first formula"))
}
