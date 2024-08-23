use std::fmt::Debug;

use anyhow::Result;
use dyn_clone::DynClone;
use hermes_five::animation::Track;
use serde::{Deserialize, Serialize};

use crate::animation::group::Group;
use crate::hardware::board::Board;
use crate::impl_entity;
use crate::utils::database::Database;
use crate::utils::entity::Entity;
use crate::utils::entity::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    // Delete the associated group if any.
    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        // 1. Delete the associated group if any.
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
    fn reset(&mut self, board: &Board) -> Result<()>;
    fn set_state(&mut self, state: u16) -> Result<u16>;
    fn into_track(&self) -> Option<Track>;
}
dyn_clone::clone_trait_object!(DeviceType);

/// Helper macro to implement a [`Device`] for a given hermes_five device type.
#[macro_export]
macro_rules! impl_device {
    ($struct_name:ident $(, { $($additional_impl:item)* })?) => {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct $struct_name {
            #[serde(flatten)]
            pub inner: hermes_five::devices::$struct_name,
        }

        impl Deref for $struct_name {
            type Target = hermes_five::devices::$struct_name;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        #[typetag::serde]
        impl DeviceType for $struct_name {
            fn set_state(&mut self, state: u16) -> anyhow::Result<u16> {
                self.inner.set_state(state)?;
                Ok(state)
            }

            // Apply additional methods if provided
            $(
                $($additional_impl)*
            )?
        }
    };
}
