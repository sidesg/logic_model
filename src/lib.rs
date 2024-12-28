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
    pub fn all_true() -> ModalOptions {
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
    pub  fn new(options: ModalOptions, formulas: Vec<String>) -> Model {
        let world0 = Rc::new(World::first_world());
        let mut worlds = HashMap::new();
        worlds.insert(0, Rc::clone(&world0));
        
        Model {
            worlds: worlds,
            modal_options: options,
            wrw: None,
            tableau: Branch::make_root(formulas, &world0)
        }
    }
    
    fn add_world(&mut self, from: World) {
        todo!()
    }

    fn update_wrw(&mut self) {
        todo!()
    }

}

#[test]
fn test_test() {
    let rootformulas: Vec<String> = vec![
        String::from("first formula"),
        String::from("second formula"),
        String::from("third formula")
    ];
    let model = Model::new(ModalOptions::all_true(), rootformulas);
}
