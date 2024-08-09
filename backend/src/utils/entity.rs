use std::any::{Any, type_name};

use anyhow::Result;
use dyn_clone::DynClone;

use crate::utils::database::{ArcDb, Database};
use crate::utils::entity::private_entity::EntityToAny;

pub type EntityType = String;
pub type Id = usize;

/// The [`Entity`] trait allows a structure to be stored in the database.
#[typetag::serde(tag = "entity")]
pub trait Entity: DynClone + Any + Send + Sync + EntityToAny {
    /// Exposes the entity id.
    fn get_id(&self) -> Id;

    /// (internal)
    /// /!\ You should never use this.
    fn set_id(&mut self, id: Id);

    /// Retrieves the entity type.
    fn get_entity_type() -> EntityType
    where
        Self: Sized,
    {
        type_name::<Self>().split("::").last().unwrap().to_string()
    }

    /// (internal)
    /// Workaround: We would need custom implementation of serialize/deserialize for storage.
    /// In the absence of a found solution at the moment, this method is used to post-process the deserialized of entities.
    /// @todo find a better solution
    /// @todo remove when https://github.com/serde-rs/serde/issues/642
    fn post_load(&mut self, _: &Database) -> Result<()> {
        Ok(())
    }

    /// Find entity by Id.
    fn get(database: &ArcDb, id: &Id) -> Result<Option<Self>>
    where
        Self: Sized + Clone,
    {
        database.read().get::<Self>(id)
    }

    /// Saves the entity into the storage.
    fn save(self, database: &ArcDb) -> Result<Self>
    where
        Self: Sized + Clone,
    {
        database.write().set(self)
    }
}
// Makes a Box<dyn DynClone> clone (used for Database cloning).
dyn_clone::clone_trait_object!(Entity);

pub(crate) mod private_entity {
    use std::any::Any;

    pub trait EntityToAny: 'static {
        fn as_any(&self) -> &dyn Any;
    }

    impl<T: 'static> EntityToAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

/// Helper macro to implement an [`Entity`] for a given structure.
#[macro_export]
macro_rules! impl_entity {
    ($struct_name:ident $(, { $($additional_impl:item)* })?) => {
        #[typetag::serde]
        impl Entity for $struct_name {
            fn get_id(&self) -> Id {
                self.id
            }

            fn set_id(&mut self, id: Id) {
                self.id = id
            }

            // Apply additional methods if provided
            $(
                $($additional_impl)*
            )?
        }
    };
}
