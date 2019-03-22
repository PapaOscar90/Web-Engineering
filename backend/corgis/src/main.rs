mod airlines;

use airlines::DataSet;

fn main() {
    let data_set = DataSet::new();

    for airport in data_set.airports() {
        println!("{:?}", airport);
    }

    for carrier in data_set.carriers() {
        println!("{:?}", carrier);
    }

    println!("{}", data_set.records().count());
}
