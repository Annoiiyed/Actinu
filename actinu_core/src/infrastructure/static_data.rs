use uuid::Uuid;

use crate::{
    domain::crud_repository::BaseRepository, domain::organisation::Organisation, ActinuError,
};

#[derive(serde::Deserialize)]
pub struct StaticData {
    organisations: Vec<Organisation>,
}

impl BaseRepository<Organisation> for StaticData {
    fn get_all(&self) -> Result<Vec<Organisation>, ActinuError> {
        Ok(self.organisations.clone())
    }

    fn get_by_ids(&self, _ids: &Vec<Uuid>) -> Result<Vec<Organisation>, ActinuError> {
        todo!()
    }

    fn create(&self, _ent: Organisation) -> Result<(), ActinuError> {
        Err(ActinuError::Internal {
            reason: "Cannot write static data",
        })
    }

    fn update(&self, _ent: Organisation) -> Result<(), ActinuError> {
        Err(ActinuError::Internal {
            reason: "Cannot write static data",
        })
    }

    fn delete(&self, _ent: Organisation) -> Result<(), ActinuError> {
        Err(ActinuError::Internal {
            reason: "Cannot write static data",
        })
    }
}
