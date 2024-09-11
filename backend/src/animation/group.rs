use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::hardware::device::Device;
use crate::impl_entity;
use crate::utils::database::Database;
use crate::utils::entity::Entity;
use crate::utils::entity::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Id,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    pub order: usize,
    pub children: Vec<Id>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub device: Option<Id>,
}
impl_entity!(Group, {
    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        // Delete the associated device if any.
        if self.device.is_some() {
            let device = database.get::<Device>(&self.device.unwrap())?;
            if device.is_some() {
                database.delete::<Device>(device.unwrap().id)?;
            };
        }

        // Update the parent group if any.
        let parent = database
            .list::<Group>()?
            .into_iter()
            .find(|(_, group)| group.children.contains(&self.id));

        if parent.is_some() {
            let (_, mut parent) = parent.unwrap();
            parent.children.retain(|id| id != &self.id);
            database.update(parent)?;
        };

        Ok(())
    }
});

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: 0,
            name: Some(name),
            order: 0,
            children: vec![],
            device: None,
        }
    }
}
