#![feature(proc_macro_hygiene, decl_macro)]
use corgis::airlines::*;

fn rocket() -> rocket::Rocket {
    rocket::ignite().manage(DataStore::new())
}

fn main() {
    rocket().launch();
}
