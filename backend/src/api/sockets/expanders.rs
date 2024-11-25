use crate::animations::Group;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::devices::Device;
use crate::hardware::{Expander, HardwareType};
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};
use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

pub fn register_expander_events(socket: &SocketRef) {
    socket.on(
        "expander:list",
        |State(database): State<ArcDb>, ack: AckSender| {
            debug!("Event received: [expander:list]");
            let expanders = database.read().list::<Expander>();
            ack.send(&Ack::from(expanders)).ok();
        },
    );

    socket.on(
        "expander:reset",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>| {
            debug!("Event received: [expander:reset]: expander:{}", id);
            database.write().set_autosave(false);

            let _ = database.read().list::<Device>().and_then(|mut devices| {
                for (_, device) in &mut devices {
                    if let HardwareType::Expander(hid) = device.hid {
                        if hid == id {
                            device.inner.reset().and_then(|mutation| {
                                broadcast_to_all(
                                    "device:mutated",
                                    Ok((device.id, mutation)),
                                    &socket,
                                );
                                Ok(())
                            })?;
                        }
                    }
                }

                Ok(devices)
            });
            database.write().set_autosave(true);
        },
    );

    socket.on(
        "expander:create",
        |socket: SocketRef,
         TryData(new_expander): TryData<Expander>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!(
                "Event received: [expander:create]: expander:{:#?}",
                new_expander
            );

            let expander = match new_expander {
                Err(error) => Err(anyhow!("Invalid expander: {}", error)),
                Ok(new_expander) => database.write().insert(new_expander),
            };
            broadcast_and_ack("expander:updated", expander, &socket, ack);
        },
    );

    socket.on(
        "expander:update",
        |socket: SocketRef,
         TryData(expander): TryData<Expander>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!(
                "Event received: [expander:update]: expander:{:#?}",
                expander
            );

            let expander = match expander {
                Err(error) => Err(anyhow!("Invalid expander: {}", error)),
                Ok(expander) => {
                    Expander::get(&database, &expander.id).and_then(|existing_expander| {
                        match existing_expander {
                            None => bail!("Expander [{}] not found", expander.id),
                            Some(_) => database.write().update(expander),
                        }
                    })
                }
            };
            broadcast_and_ack("expander:updated", expander, &socket, ack);
        },
    );

    socket.on(
        "expander:delete",
        |socket: SocketRef, database: State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [expander:delete]: id:{:?}", id);
            let expander = database
                .write()
                .delete::<Expander>(id)
                .and_then(|expander| match expander {
                    None => bail!("Expander not found"),
                    Some(expander) => Ok(expander),
                });

            let devices = database.read().list::<Device>();
            broadcast_to_all("device:list", devices, &socket);
            let groups = database.read().list::<Group>();
            broadcast_to_all("group:list", groups, &socket);
            broadcast_and_ack("expander:deleted", expander, &socket, ack);
        },
    );
}
