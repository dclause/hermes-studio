//! This module contains all the code to define `Storage`.
//!
//! The principle of storage is to provide a structure where elements implementing the `Entity` trait
//! can be stored.
//!
//! The storage is globally accessible by other services. For instance the socket and rest API
//! can't work if no storage of some sort is provided.
//!
//! Currently, the Storage comes in two flavor:
//! - volatile: purely in memory (thus resets at every app start)
//! - persistent: the data are serialized in JSON file

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use parking_lot::RwLock;

use crate::utils::entity::{Entity, EntityType, Id};

pub type ArcDb = Arc<RwLock<Database>>;

/// Storage structure: stores all data accessible via the API.
#[derive(Clone)]
pub struct Database {
    /// Path to the destination storage folder.
    /// If given, the `.save()` function will be able to serialize the content into that folder.
    destination: Option<PathBuf>,
    /// Flag to indicate if `.save()` should be automatically done after every CRUD operation.
    /// By opposition, when set to `false` no persistent storage is done without a manual call to `.save()`.
    autosave: bool,
    /// Stores the entities in memory.
    entities: HashMap<EntityType, HashMap<Id, Box<dyn Entity>>>,
}

impl Database {
    /// Initializes the storage a volatile 'in-memory' only storage.
    pub fn init_volatile() -> Result<Self> {
        Ok(Self {
            destination: None,
            autosave: false,
            entities: Default::default(),
        })
    }

    /// Initializes the storage a persisted 'in-file' storage.
    ///
    /// If the `folder` does not exist: it gets created and initializes empty.
    /// If the `folder` exists:
    ///     - if `reset_if_exists` flag is `true`: the folder is emptied.
    ///     - if `reset_if_exists` flag is `false`: the content is extracted from the files into the Storage.
    ///
    /// If the `autosave` parameter is set to `true`: the storage content will be dumped to the file system
    /// everytime something relevant is created / updated / deleted.
    /// If the `autosave` parameter is set to `false`: the storage will work exclusively in memory (faster)
    /// but it is the programmer responsibility to save it manually via the `save` method.
    pub fn init_persistent<P: AsRef<Path>>(
        folder: P,
        reset_if_exists: bool,
        autosave: bool,
    ) -> Result<Self> {
        let path = folder.as_ref();
        // Check path validity.
        if !path.is_dir() && path.exists() {
            bail!("Provided destination is not a directory.");
        }

        // Make sure the path exists.
        std::fs::create_dir_all(&path)?;

        let mut storage = Database {
            destination: Some(PathBuf::from(path)),
            autosave,
            entities: Default::default(),
        };

        // Reset the storage if necessary / Load content otherwise.
        if reset_if_exists {
            std::fs::remove_dir_all(path)?;
            std::fs::create_dir_all(path)?;
        } else {
            storage.load()?;
        }

        Ok(storage)
    }

    pub fn set_autosave(&mut self, autosave: bool) {
        self.autosave = autosave;
    }

    /// Dumps the storage content to the persistent storage if available.
    ///
    /// If no destination was given when the storage was created
    /// (using `Storage::from(...)`) an `Err` is raised.
    pub fn dump(&self) -> Result<()> {
        let entities = &self.entities;
        for (entity_type, _) in entities.iter() {
            self.save_to_file(entity_type)?;
        }

        Ok(())
    }

    /// (private)
    /// Loads content from the persistent storage if available.
    fn load(&mut self) -> Result<()> {
        if let Some(destination) = &self.destination {
            let files = std::fs::read_dir(destination)?;

            for file in files {
                let file = file?.path();

                // Skip non json files.
                if file.extension().is_none() || file.extension().unwrap() != "json" {
                    continue;
                }

                let entity_type: EntityType = file
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .strip_suffix('s')
                    .unwrap()
                    .to_string();

                // Deserialize data from the current entity_type file in the storage folder.
                let data = std::fs::read_to_string(file)?;
                let entities = serde_json::from_str::<HashMap<Id, Box<dyn Entity>>>(data.as_str())?;

                // Update the storage to save the provided entity
                self.entities.insert(entity_type, entities);
            }

            // Iterate over all entities and call post_load
            let cloned_db = self.clone();
            for (_, entity_list) in self.entities.iter_mut() {
                for (_, entity) in entity_list {
                    entity.post_load(&cloned_db)?;
                }
            }
        }
        Ok(())
    }

    /// Retrieves all the entities of an entity_type stored in the storage.
    ///
    /// This method is private to this crate only since developers should rather
    /// use the entities CRUD methods instead of directly using the storage.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use hermes_core::hardware::boards::arduino::{ArduinoBoard, ArduinoModel};
    /// use hermes_core::hardware::boards::Board;
    /// use hermes_core::storage::entity::{Entity, Id};
    /// use hermes_core::storage::storage::Storage;
    /// Storage::init_volatile().expect("Storage init");
    /// ArduinoBoard::build("Board1", ArduinoModel::MEGA).save().expect("Error saving the entity");
    /// ArduinoBoard::build("Board2", ArduinoModel::MEGA).save().expect("Error saving the entity");
    /// let boards: HashMap<Id, Board> = Board::list().expect("panic!");
    /// assert_eq!(boards.keys().len(), 2);
    /// ```
    pub fn list<T: Entity + Clone + 'static>(&self) -> Result<HashMap<Id, T>> {
        let entity_type = T::get_entity_type();
        let entities = self
            .entities
            .get(&entity_type)
            .map_or(HashMap::new(), |entities| {
                entities
                    .iter()
                    .map(|(id, entity)| {
                        let entity = entity.deref().as_any().downcast_ref::<T>();
                        (*id, entity.unwrap().clone())
                    })
                    .collect()
            });

        Ok(entities)
    }

    // pub fn list<T: Entity + Clone + 'static>(&self) -> Result<HashMap<Id, T>> {
    //     let entity_type = T::get_entity_type();
    //     let entities = self
    //         .entities
    //         .get(&entity_type)
    //         .map_or(HashMap::new(), |entities| {
    //             entities
    //                 .iter()
    //                 .map(|(id, entity)| {
    //                     let entity = entity.deref().as_any().downcast_ref::<T>();
    //                     (*id, entity.unwrap().clone())
    //                 })
    //                 .collect()
    //         });
    //
    //     Ok(entities)
    // }

    /// Retrieves an entity stored in the storage.
    ///
    /// This method is private to this crate only since developers should rather
    /// use the entities CRUD methods instead of directly using the storage.
    ///
    /// # Examples
    /// ```
    /// use hermes_core::hardware::boards::arduino::{ArduinoBoard, ArduinoModel};
    /// use hermes_core::hardware::boards::Board;
    /// use hermes_core::storage::entity::Entity;
    /// use hermes_core::storage::storage::Storage;
    /// Storage::init_volatile().expect("Storage init");
    /// let entity = ArduinoBoard::build("Board", ArduinoModel::MEGA).save().expect("Error saving the entity");
    /// let retrieved_entity = Board::get(&entity.get_id()).expect("Error using the storage");
    /// assert!(retrieved_entity.is_some());
    /// ```
    pub fn get<T: Entity + Clone + 'static>(&self, id: &Id) -> Result<Option<T>> {
        let entity_type = T::get_entity_type();

        let entity = self
            .entities
            .get(&entity_type)
            .and_then(|entities| entities.get(id))
            .map_or(None, |entity| {
                let entity = entity.deref().as_any().downcast_ref::<T>();
                entity.cloned()
            });

        Ok(entity)
    }

    /// Stores or Updates an entity stored in the storage.
    pub(crate) fn set<T: Entity + 'static + Clone>(&mut self, mut entity: T) -> Result<T> {
        let entity_type = T::get_entity_type();
        let entities = self
            .entities
            .entry(entity_type.clone())
            .or_insert_with(HashMap::new);

        // Get the entity id or generate if not set.
        let id = match entity.get_id() {
            0 => entities.keys().max().map_or(1, |id| id + 1),
            id => id,
        };

        entity.set_id(id);
        entities.insert(id, Box::new(entity.clone()));

        // Optionally, triggers autosave if enabled:
        // this will save the entities of the current type to its dump file.
        if self.autosave {
            self.save_to_file(&entity_type)?;
        }

        // Run post_save hook
        entity.post_save(self)?;

        Ok(entity)
    }

    /// Inserts a new entity in the storage.
    pub fn insert<T: Entity + 'static + Clone>(&mut self, entity: T) -> Result<T> {
        // Get the entity id or generate if not set.
        if entity.get_id() > 0 {
            bail!("Cannot insert an entity that already have an id");
        }
        self.set(entity)
    }

    /// Updates an existing entity in the storage.
    pub fn update<T: Entity + 'static + Clone>(&mut self, entity: T) -> Result<T> {
        if self.get::<T>(&entity.get_id())?.is_none() {
            bail!("No entity found with this ID.")
        };
        self.set(entity)
    }

    /// Deletes an entity stored from the storage.
    ///
    /// This method is private to this crate only since developers should rather
    /// use the entities CRUD methods instead of directly using the storage.
    ///
    /// # Examples
    /// ```
    /// use hermes_core::hardware::boards::arduino::{ArduinoBoard, ArduinoModel};
    /// use hermes_core::hardware::boards::Board;
    /// use hermes_core::storage::entity::Entity;
    /// use hermes_core::storage::storage::Storage;
    /// Storage::init_volatile().expect("Storage init");
    ///
    /// // Delete from the entity.
    /// let entity = ArduinoBoard::build("Board", ArduinoModel::MEGA).save().expect("panic!");
    /// assert!(entity.delete().is_ok());
    ///
    /// // Static delete by Id is also possible.
    /// let entity = ArduinoBoard::build("Board", ArduinoModel::MEGA).save().expect("panic!");
    /// assert!(Board::delete_by_id(entity.get_id()).is_ok());
    /// ```
    pub fn delete<T: Entity + 'static + Clone>(&mut self, id: Id) -> Result<Option<T>> {
        let entity_type = T::get_entity_type();
        let entities = self
            .entities
            .entry(entity_type.clone())
            .or_insert_with(HashMap::new);

        let entity = entities.remove(&id).map_or(None, |entity| {
            let entity = entity.deref().as_any().downcast_ref::<T>();
            entity.cloned()
        });

        // Optionally, triggers autosave if enabled:
        // this will save the entities of the current type to its dump file.
        if entity.is_some() && self.autosave {
            // Run post_delete hook.
            entity.clone().unwrap().post_delete(self)?;
            self.save_to_file(&entity_type)?;
        }

        Ok(entity)
    }

    pub fn save_to_file(&self, entity_type: &EntityType) -> Result<()> {
        let path = self
            .destination
            .as_ref()
            .ok_or_else(|| anyhow!("Destination folder undefined."))?;

        let entities = self
            .entities
            .get(entity_type)
            .ok_or_else(|| anyhow!("Entity type undefined."))?;

        let serialized_entities = serde_json::to_string_pretty(&entities)?;

        let entity_filepath = path.join(format!("{}s.json", entity_type));
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(entity_filepath)?;
        file.write_all(serialized_entities.as_ref())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};

    use super::*;

    // For setting Unix permissions

    #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
    struct MockEntity {
        id: Id,
    }
    #[typetag::serde]
    impl Entity for MockEntity {
        fn get_id(&self) -> Id {
            self.id
        }

        fn set_id(&mut self, id: Id) {
            self.id = id;
        }
    }

    #[test]
    fn test_init_volatile() {
        let db = Database::init_volatile().expect("Failed to initialize volatile storage");
        assert_eq!(db.destination, None);
        assert_eq!(db.autosave, false);
    }

    #[test]
    fn test_init_persistent_create_new() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db = Database::init_persistent(temp_dir.path(), true, false)
            .expect("Failed to initialize persistent storage");
        assert!(db.destination.is_some());
        assert_eq!(db.autosave, false);
    }

    #[test]
    fn test_init_persistent_load_existing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut db = Database::init_persistent(temp_dir.path(), true, true)
            .expect("Failed to initialize persistent storage");

        // Add an entity (and auto-save)
        let entity = MockEntity::default();
        db.set(entity.clone()).expect("Failed to set entity");

        // Reinitialize and load the existing storage
        let mut db_loaded = Database::init_persistent(temp_dir.path(), false, false)
            .expect("Failed to initialize persistent storage");
        let entities: HashMap<Id, MockEntity> = db_loaded.list().expect("Failed to list entities");
        assert_eq!(entities.len(), 1);
    }

    #[test]
    fn test_load_with_nonexistent_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut db = Database::init_persistent(temp_dir.path(), true, false)
            .expect("Failed to initialize persistent storage");
        std::fs::remove_dir_all(temp_dir.path()).unwrap(); // Simulate directory deletion
        let result = db.load();
        assert!(result.is_err()); // Expect error due to missing directory
    }

    #[test]
    fn test_load_with_invalid_file_content() {
        let temp_dir = tempfile::tempdir().unwrap();

        let mut db = Database::init_persistent(temp_dir.path(), true, true)
            .expect("Failed to initialize persistent storage");
        let entity = MockEntity::default();
        db.set(entity).expect("save");

        let path = temp_dir.path().join("MockEntity.json");
        assert!(path.exists());
        std::fs::write(&path, "invalid json").unwrap(); // Write invalid content

        let result = db.load();
        assert!(result.is_err()); // Expect error due to invalid JSON
    }

    #[test]
    fn test_set_and_get_entity() {
        let mut db = Database::init_volatile().expect("Failed to initialize volatile storage");
        let entity = MockEntity::default();
        let saved_entity = db.set(entity.clone()).expect("Failed to save entity");
        assert_eq!(saved_entity.get_id(), 1); // The ID should be updated to 1
        let retrieved_entity = db
            .get::<MockEntity>(&saved_entity.get_id())
            .expect("Failed to get entity");
    }

    #[test]
    fn test_get_with_nonexistent_entity() {
        let db = Database::init_volatile().expect("Failed to initialize volatile storage");
        let result = db.get::<MockEntity>(&1);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Expect no entity found
    }

    #[test]
    fn test_delete_entity() {
        let mut db = Database::init_volatile().expect("Failed to initialize volatile storage");
        let entity = MockEntity::default();
        let saved_entity = db.set(entity.clone()).expect("Failed to save entity");
        assert_eq!(saved_entity.get_id(), 1);
        let deleted_entity = db.delete::<MockEntity>(1).expect("Failed to delete entity");
        assert!(deleted_entity.is_some());
        let retrieved_entity = db.get::<MockEntity>(&1).expect("Failed to get entity");
        assert!(retrieved_entity.is_none());
    }

    #[test]
    fn test_delete_with_nonexistent_entity() {
        let mut db = Database::init_volatile().expect("Failed to initialize volatile storage");
        let result = db.delete::<MockEntity>(1);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Expect no entity found
    }

    #[test]
    fn test_dump_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut db = Database::init_persistent(temp_dir.path(), true, true)
            .expect("Failed to initialize persistent storage");

        let entity = MockEntity::default();
        db.set(entity.clone()).expect("Failed to save entity");
        db.dump().expect("Failed to dump storage");

        // Reload database
        let mut db_reloaded = Database::init_persistent(temp_dir.path(), false, false)
            .expect("Failed to initialize persistent storage");
        let entities: HashMap<Id, MockEntity> =
            db_reloaded.list().expect("Failed to list entities");
        assert_eq!(entities.len(), 1);
    }

    #[test]
    fn test_dump_without_destination() {
        let db = Database::init_volatile().expect("Failed to initialize volatile storage");
        let result = db.dump();
        assert!(result.is_err());
    }
}
