use itertools::Itertools;

use super::super::_action::Action;
use crate::{
    domain::{crud_repository::BaseRepository, organisation::Organisation},
    ActinuError,
};

pub struct GetAll {
    organisation_repositories: Vec<Box<dyn BaseRepository<Organisation>>>,
}

impl GetAll {
    pub fn new(organisation_repositories: Vec<Box<dyn BaseRepository<Organisation>>>) -> Self {
        Self {
            organisation_repositories,
        }
    }
}

impl Action<Vec<Organisation>> for GetAll {
    fn execute(&self) -> Result<Vec<Organisation>, ActinuError> {
        self.organisation_repositories
            .iter()
            .map(|repo| repo.get_all())
            .flatten_ok()
            .collect()
    }
}
