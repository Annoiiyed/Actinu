use crate::data_source::get_static_data;
use actinu_core::organisations::get_all::GetAll;
use actinu_core::prelude::*;

pub fn run() {
    let repositories = vec![get_static_data()];

    let r = GetAll::new(repositories).execute();

    println!("Received: {:?}", r);
}
