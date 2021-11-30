//#![deny(unused_imports)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use log::*;
use simplelog::*;

use std::fs::File;

pub mod binary;
pub mod protocol;
pub mod simulation;
pub mod types;

type Arena = Arc<RwLock<simulation::Arena>>;

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("kanono.log").unwrap(),
        ),
    ])
    .unwrap();

    let arena = Arc::new(RwLock::new(simulation::Arena::new(4000, 4000)));

    let arena_filter = {
        let arena = arena.clone();
        warp::any().map(move || arena.clone())
    };

    let wss = warp::path("ws")
        .and(warp::ws())
        .and(arena_filter)
        .map(|ws: warp::ws::Ws, arena| {
            ws.on_upgrade(move |socket| handle_connection(socket, arena))
        });

    {
        let arena = arena.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(33));
            loop {
                arena.clone().write().await.update();
                interval.tick().await;
            }
        });
    }

    warp::serve(wss).run(([127, 0, 0, 1], 3000)).await;
}

async fn handle_connection(ws: WebSocket, arena: Arena) {
    info!("New connection: {:?}", ws);

    let mut id: Option<types::Identifer> = None;

    // Split the socket into a sender and receive of messages.
    let (mut ws_tx, mut ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            match ws_tx.send(message).await {
                Ok(_) => {}
                Err(_) => break,
            }
        }
    });

    tx.send(warp::ws::Message::binary(
        protocol::ClientboundPacket::RoomInfo {
            width: 4000,
            height: 4000,
            mode: "ffa".to_string(),
            accounts_enabled: true,
            border_style: 0,
        }
        .to_bytes(),
    ))
    .unwrap();

    tx.send(warp::ws::Message::binary(
        protocol::ClientboundPacket::EntityTypes(include_str!("tanks.json").to_string()).to_bytes(),
    ))
    .unwrap();

    tx.send(warp::ws::Message::binary(
        protocol::ClientboundPacket::Message {
            message: "Welcome to Kanono: Global Offensive".to_string(),
            color: types::Color::Black,
        }
        .to_bytes(),
    ))
    .unwrap();

    id = Some(arena.write().await.new_connection(tx.clone()));
    tx.send(warp::ws::Message::binary(
        protocol::ClientboundPacket::Identifier(id.unwrap() as u32).to_bytes(),
    ))
    .unwrap();

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error(uid={:?}): {}", id, e);
                break;
            }
        };

        if !msg.is_binary() {
            info!(
                "closing socket for sending non-binary(uid={:?}): {:?}",
                id, msg
            );
            dbg!(arena.write().await.kick_connection(id.unwrap()));
            break;
        }
        let msg = msg.as_bytes();
        let parsed = protocol::ServerboundPacket::from_bytes(msg);

        match parsed {
            Ok(protocol::ServerboundPacket::Spawn(name)) => {
                if !arena.write().await.player_spawn(id.unwrap(), name.clone()) {
                    break;
                }
                info!("Got spawn packet(uid={:?}): {}", id, name);
            }
            Ok(protocol::ServerboundPacket::Input {
                left,
                right,
                up,
                down,
                lmb,
                angle,
                mx,
                my,
                rmb,
            }) => {
                arena.write().await.input(
                    id.unwrap(),
                    left,
                    right,
                    up,
                    down,
                    angle,
                    lmb,
                    mx,
                    my,
                    rmb,
                );
            }
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Error decoding message, closing socket(uid={:?}): {:?}",
                    id, e
                );
                arena.write().await.kick_connection(id.unwrap());
                break;
            }
        }
    }
}
