pub mod crud_repository;
pub mod organisation;

use uuid::Uuid;

pub struct Role {
    id: Uuid,
    name: String,
}

impl Role {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

pub struct CrudData {
    created_by: Uuid,
    created_on: String,
}

impl CrudData {
    pub fn new(created_by: Uuid, created_on: String) -> Self {
        Self {
            created_by,
            created_on,
        }
    }
}

pub struct Person {
    id: Uuid,
    name: String,
    organisation: Uuid,
    roles: Vec<Role>,
    crud: Box<CrudData>,
}

impl Person {
    pub fn new(
        id: Uuid,
        name: String,
        organisation: Uuid,
        roles: Vec<Role>,
        crud: Box<CrudData>,
    ) -> Self {
        Self {
            id,
            name,
            organisation,
            roles,
            crud,
        }
    }
}

pub struct Document {
    id: Uuid,
    name: String,
    crud: CrudData,
}

impl Document {
    pub fn new(id: Uuid, name: String, crud: CrudData) -> Self {
        Self { id, name, crud }
    }
}
