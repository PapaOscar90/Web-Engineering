use corgis_dataset::airlines::DataSet;

fn main() {
    let data_set = DataSet::new();

    println!("Displaying airports:");
    let airport_bound = data_set.airports().len() - 1;
    for (idx, airport) in data_set.airports().enumerate() {
        println!("{}/{} {:#?}", idx, airport_bound, airport);
    }

    println!("Displaying carriers:");
    let carrier_bound = data_set.carriers().len() - 1;
    for (idx, carrier) in data_set.carriers().enumerate() {
        println!("{}/{} {:#?}", idx, carrier_bound, carrier);
    }
}
