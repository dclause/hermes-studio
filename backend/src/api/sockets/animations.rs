use crate::animations::animation::Animation;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::storage::entity::{Entity, Id};
use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, SocketRef, TryData};

pub fn register_animation_events(socket: &SocketRef) {
    socket.on("animation:list", |ack: AckSender| {
        debug!("Event received: [animation:list]");
        ack.send(Ack::from(Animation::list())).ok();
    });

    socket.on(
        "animation:create",
        |socket: SocketRef, TryData(new_animation): TryData<Animation>, ack: AckSender| {
            debug!(
                "Event received: [animation:create]: animation:{:?}",
                new_animation
            );

            let animation = match new_animation {
                Ok(animation) => animation.save(),
                Err(error) => Err(anyhow!("Invalid animation: {}", error)),
            };

            broadcast_and_ack("animation:updated", animation, socket, ack);
        },
    );

    socket.on(
        "animation:update",
        |socket: SocketRef, TryData(animation): TryData<Animation>, ack: AckSender| {
            debug!(
                "Event received: [animation:update]: animation:{:?}",
                animation
            );

            let animation = match animation {
                Ok(animation) => animation.save(),
                Err(error) => Err(anyhow!("Invalid animation: {}", error)),
            };
            broadcast_and_ack("animation:updated", animation, socket, ack);
        },
    );

    socket.on(
        "animation:delete",
        |socket: SocketRef, TryData(id): TryData<Id>, ack: AckSender| {
            debug!("Event received: [animation:delete]: id:{:?}", id);
            let animation =
                Animation::delete_by_id(id.unwrap()).and_then(|animation| match animation {
                    None => bail!("Animation not found"),
                    Some(animation) => Ok(animation),
                });
            broadcast_and_ack("animation:deleted", animation, socket, ack);
        },
    );
}
