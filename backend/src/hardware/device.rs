use std::fmt::Debug;

use anyhow::Result;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::animation::groups::Group;
use crate::hardware::board::Board;
use crate::impl_entity;
use crate::utils::database::Database;
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

impl_entity!(Device, {
    fn post_save(&mut self, database: &mut Database) -> Result<()> {
        if database
            .list::<Group>()?
            .iter()
            .find(|(_, group)| group.device.is_some() && group.device.unwrap() == self.id)
            .is_none()
        {
            let group = Group {
                id: 0,
                name: None,
                order: 0,
                children: vec![],
                device: Some(self.id),
            };
            database.insert(group)?;
        };
        Ok(())
    }

    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        let group = database
            .list::<Group>()?
            .into_iter()
            .find(|(_, group)| group.device.is_some() && group.device.unwrap() == self.id);

        if group.is_some() {
            database.delete::<Group>(group.unwrap().1.id)?;
        };
        Ok(())
    }
});

#[typetag::serde(tag = "type")]
pub trait DeviceType: DynClone + Debug + Send + Sync {
    fn init(&mut self, board: &Board) -> Result<()>;
    fn set_state(&mut self, state: u16) -> Result<u16>;
}
dyn_clone::clone_trait_object!(DeviceType);
