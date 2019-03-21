mod airlines;

use airlines::DataSet;

fn main() {
    let data_set = DataSet::new();
    println!("{:#?}", data_set);
}
