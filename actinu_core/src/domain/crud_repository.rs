use uuid::Uuid;

use crate::ActinuError;

pub trait BaseRepository<T> {
    fn get_all(&self) -> Result<Vec<T>, ActinuError>;
    fn get_by_ids(&self, ids: &Vec<Uuid>) -> Result<Vec<T>, ActinuError>;
    fn create(&self, ent: T) -> Result<(), ActinuError>;
    fn update(&self, ent: T) -> Result<(), ActinuError>;
    fn delete(&self, ent: T) -> Result<(), ActinuError>;
}
