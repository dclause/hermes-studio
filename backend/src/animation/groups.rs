use serde::{Deserialize, Serialize};

use crate::impl_entity;
use crate::utils::entity::Entity;
use crate::utils::entity::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub order: usize,
    pub children: Vec<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Id>,
}
impl_entity!(Group);
