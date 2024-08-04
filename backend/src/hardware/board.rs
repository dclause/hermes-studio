use hermes_five::Board as InnerBoard;
use serde::{Deserialize, Serialize};

use crate::impl_entity;
use crate::utils::entity::{Entity, Id};

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: Id,
    pub name: String,
    pub inner: InnerBoard,
}

impl_entity!(Board);
impl Board {}
