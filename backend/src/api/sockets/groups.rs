use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::group::Group;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::api::sockets::ack::Ack;
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
        "group:create",
        |socket: SocketRef,
         TryData(name): TryData<String>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [group:create]: group:{:?}", name);

            let group = match name {
                Ok(name) => database.write().insert(Group::new(name)),
                Err(error) => Err(anyhow!("Invalid group: {}", error)),
            };
            broadcast_and_ack("group:updated", group, &socket, ack);
        },
    );

    socket.on(
        "group:update",
        |socket: SocketRef,
         TryData(data): TryData<(Id, String)>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [group:update]: group:{:?}", data);

            let group = match data {
                Ok(data) => {
                    let (id, name) = data;
                    Group::get(&database, &id).and_then(|group| match group {
                        None => bail!("Group not found"),
                        Some(mut group) => {
                            group.name = Some(name);
                            database.write().update(group)
                        }
                    })
                }
                Err(error) => Err(anyhow!("Invalid group: {}", error)),
            };
            broadcast_and_ack("group:updated", group, &socket, ack);
        },
    );

    socket.on(
        "groups:save",
        |socket: SocketRef,
         TryData(groups): TryData<HashMap<Id, Group>>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [groups:save]");

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
            broadcast_and_ack("group:list", groups, &socket, ack);
        },
    );

    socket.on(
        "group:delete",
        |socket: SocketRef, database: State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [group:delete]: id:{:?}", id);
            let group = database
                .write()
                .delete::<Group>(id)
                .and_then(|group| match group {
                    None => bail!("Group not found"),
                    Some(group) => Ok(group),
                });
            ack.send(Ack::from(group)).ok();

            let groups = database.read().list::<Group>();
            broadcast_to_all("group:list", groups, &socket);
        },
    );
}
