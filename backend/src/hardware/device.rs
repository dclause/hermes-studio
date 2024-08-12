use std::fmt::Debug;

use anyhow::Result;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::impl_entity;
use crate::utils::entity::Entity;
use crate::utils::entity::Id;

#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Id,
    pub bid: Id,
    pub name: String,
    #[serde(flatten)]
    pub inner: Box<dyn DeviceType>,
}

impl_entity!(Device);

#[typetag::serde(tag = "type")]
pub trait DeviceType: DynClone + Debug + Send + Sync {
    fn init(&mut self, board: &Board) -> Result<()>;
    fn set_state(&mut self, state: u16) -> Result<u16>;
}
dyn_clone::clone_trait_object!(DeviceType);
