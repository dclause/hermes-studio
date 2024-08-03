use std::any::{Any, type_name};
use std::fmt::Debug;

use crate::utils::entity::private_entity::EntityToAny;

pub type EntityType = String;
pub type Id = usize;

#[typetag::serde(tag = "type")]
pub trait Entity: Debug + Any + Send + Sync + EntityToAny {
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
}

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
