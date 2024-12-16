#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::rc::Rc;    

mod world;
use crate::world::World;

mod branch;
use crate::branch::Branch;

struct ModalOptions {
    rho: bool,
    sigma: bool,
    tau: bool,
    eta: bool
}

impl ModalOptions {
    fn all_true() -> ModalOptions {
        ModalOptions {
            rho: true,
            sigma: true,
            tau: true,
            eta: true
        }
    }
}

struct Model {
    worlds: HashMap<u64, Rc<World>>,
    modal_options: ModalOptions,
    wrw: Option<Vec<(World, World)>>,
    tableau: Branch
}

impl Model {
    fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        let world0 = Rc::new(World::first_world());
        let mut worlds = HashMap::new();
        worlds.insert(0, Rc::clone(&world0));
        
        Model {
            worlds: worlds,
            modal_options: options,
            wrw: None,
            tableau: Branch::make_root(formulas, Rc::clone(&world0))
        }
    }
    
    fn add_world(&mut self, from: World) {
        todo!()
    }

    fn update_wrw(&mut self) {
        todo!()
    }

}

// struct Tableau {
//     model: Model,
//     tree: Vec<Node>
// }

// impl Tableau {
//     fn new(expressions: Vec<String>, model: Model) -> Tableau {
//         Tableau {
//             model: model,
//             tree: 
//                 expressions
//                     .into_iter()
//                     .map(|ex| Node::new(ex))
//                     .collect()
            
//         }
//     }

//     fn implement_rule(&mut self) {
//         let active_node = self.tree
//             .iter_mut()
//             .filter(|exp| exp.state == NodeState::Active)
//             .next()
//             .unwrap();
//         active_node.deactivate();
//     }
// }

// #[test]
// fn new_tableau() {
//     let expressions = vec!["p > q".to_string(), "therefore ~q > ~p".to_string()];
//     let options = ModalOptions::all_true();
//     let model = Model::initial_model(options);
//     let tableau = Tableau::new(expressions, model);
//     assert_eq!(tableau.tree, vec![
//         Node::new("p > q".to_string()), 
//         Node::new("therefore ~q > ~p".to_string())
//     ]);
// }

// #[test]
// fn deactivate() {
//     let expressions = vec!["p > q".to_string(), "therefore ~q > ~p".to_string()];
//     let options = ModalOptions::all_true();
//     let model = Model::initial_model(options);
//     let mut tableau = Tableau::new(expressions, model);

//     tableau.implement_rule();
//     assert_eq!(
//         tableau.tree[0].state,
//         NodeState::Inactive
//     );
    
// }
