use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::debug;
use socketioxide::extract::{AckSender, SocketRef, State, TryData};

use crate::animation::groups::Group;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};

pub fn register_group_events(socket: &SocketRef) {
    socket.on(
        "group:list",
        |State(database): State<ArcDb>, ack: AckSender| {
            debug!("Event received: [group:list]");
            let groups = database.read().list::<Group>();
            ack.send(Ack::from(groups)).ok();
        },
    );

    socket.on(
        "groups:update",
        |socket: SocketRef,
         TryData(groups): TryData<HashMap<Id, Group>>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [group:bulk_update]");

            // Disable autosave temporarily.
            database.write().set_autosave(false);

            let groups: Result<Vec<Group>> = match groups {
                Ok(groups) => groups
                    .into_iter()
                    .map(|(_, group)| database.write().set(group))
                    .collect(),
                Err(error) => Err(anyhow!("Invalid group: {}", error)),
            };

            let groups = match groups {
                Ok(_) => {
                    database
                        .write()
                        .save_to_file(&Group::get_entity_type())
                        .unwrap();
                    database.read().list::<Group>()
                }
                Err(error) => Err(anyhow!("Database error: {}", error)),
            };

            // Resume autosave
            database.write().set_autosave(true);
            broadcast_and_ack("groups:updated", groups, socket, ack);
        },
    );
    //
    // socket.on(
    //     "group:update",
    //     |socket: SocketRef, TryData(group): TryData<Group>, ack: AckSender| {
    //         debug!("Event received: [group:update]: group:{:?}", group);
    //
    //         let group = match group {
    //             Ok(group) => group.save(),
    //             Err(error) => Err(anyhow!("Invalid group: {}", error)),
    //         };
    //
    //         broadcast_and_ack("group:updated", group, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "group:delete",
    //     |socket: SocketRef, TryData(id): TryData<Id>, ack: AckSender| {
    //         debug!("Event received: [group:delete]: id:{:?}", id);
    //         let group = Group::delete_by_id(id.unwrap()).and_then(|group| match group {
    //             None => bail!("Group not found"),
    //             Some(group) => Ok(group),
    //         });
    //         broadcast_and_ack("group:deleted", group, socket, ack);
    //     },
    // );
}
