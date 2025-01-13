use logic_model::Model;

fn main() {
    let mut model = Model::from_file("data/basic.txt").unwrap();
    model.eval_tableau();
}
