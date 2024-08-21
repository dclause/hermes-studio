use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::animation::Animation;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::utils::database::ArcDb;
use crate::utils::entity::Id;

pub fn register_animation_events(socket: &SocketRef) {
    socket.on(
        "animation:list",
        |ack: AckSender, State(database): State<ArcDb>| {
            debug!("Event received: [animation:list]");
            let boards = database.read().list::<Animation>();
            ack.send(Ack::from(boards)).ok();
        },
    );

    socket.on(
        "animation:create",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(new_animation): TryData<Animation>,
         ack: AckSender| {
            debug!(
                "Event received: [animation:create]: animation:{:?}",
                new_animation
            );

            let animation = match new_animation {
                Ok(animation) => database.write().insert(animation),
                Err(error) => Err(anyhow!("Invalid animation: {}", error)),
            };

            broadcast_and_ack("animation:updated", animation, &socket, ack);
        },
    );

    socket.on(
        "animation:update",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(animation): TryData<Animation>,
         ack: AckSender| {
            debug!(
                "Event received: [animation:update]: animation:{:?}",
                animation
            );

            let animation = match animation {
                Ok(animation) => database.write().update(animation),
                Err(error) => Err(anyhow!("Invalid animation: {}", error)),
            };
            broadcast_and_ack("animation:updated", animation, &socket, ack);
        },
    );

    socket.on(
        "animation:delete",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [animation:delete]: id:{:?}", id);

            let animation = database
                .write()
                .delete::<Animation>(id)
                .and_then(|animation| match animation {
                    None => bail!("Animation not found"),
                    Some(animation) => Ok(animation),
                });

            broadcast_and_ack("animation:deleted", animation, &socket, ack);
        },
    );
}
