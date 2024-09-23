use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::posture::Posture;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::api::sockets::ack::Ack;
use crate::hardware::device::Device;
use crate::utils::database::ArcDb;
use crate::utils::entity::Id;

pub fn register_posture_events(socket: &SocketRef) {
    socket.on(
        "posture:list",
        |ack: AckSender, State(database): State<ArcDb>| {
            debug!("Event received: [posture:list]");
            let postures = database.read().list::<Posture>();
            ack.send(Ack::from(postures)).ok();
        },
    );

    socket.on(
        "posture:create",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(new_posture): TryData<Posture>,
         ack: AckSender| {
            debug!(
                "Event received: [posture:create]: posture:{:#?}",
                new_posture
            );

            let posture = match new_posture {
                Ok(posture) => database.write().insert(posture),
                Err(error) => Err(anyhow!("Invalid posture: {}", error)),
            };

            broadcast_and_ack("posture:updated", posture, &socket, ack);
        },
    );

    socket.on(
        "posture:update",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(posture): TryData<Posture>,
         ack: AckSender| {
            debug!("Event received: [posture:update]: posture:{:#?}", posture);

            let posture = match posture {
                Ok(posture) => database.write().update(posture),
                Err(error) => Err(anyhow!("Invalid posture: {}", error)),
            };
            broadcast_and_ack("posture:updated", posture, &socket, ack);
        },
    );

    socket.on(
        "posture:delete",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [posture:delete]: id:{:?}", id);

            let posture =
                database
                    .write()
                    .delete::<Posture>(id)
                    .and_then(|posture| match posture {
                        None => bail!("Posture not found"),
                        Some(group) => Ok(group),
                    });

            broadcast_and_ack("posture:deleted", posture, &socket, ack);
        },
    );

    socket.on(
        "posture:play",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [posture:play]: id:{:?}", id);

            let mut database = database.write();
            let posture = database
                .get::<Posture>(&id)
                .and_then(|posture| match posture {
                    None => bail!("Posture not found"),
                    Some(mut posture) => posture.play(&mut database),
                });

            let devices = database.list::<Device>();
            broadcast_to_all("device:list", devices, &socket);
            ack.send(Ack::from(posture)).ok();
        },
    );
}
