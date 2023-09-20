use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::crud_repository::BaseRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organisation {
    id: Uuid,
    name: String,
}

impl Organisation {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

pub trait OrganisationRepository: BaseRepository<Organisation> {}
