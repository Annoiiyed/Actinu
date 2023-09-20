use actinu_core::{
    domain::{crud_repository::BaseRepository, organisation::Organisation},
    infrastructure::static_data::StaticData,
};

pub fn get_static_data() -> Box<dyn BaseRepository<Organisation>> {
    let base_data = include_str!("base-data.json");

    Box::new(StaticData::new(base_data))
}
