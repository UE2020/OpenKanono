//#![deny(unused_imports)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use warp::Filter;

use simplelog::*;
use log::*;

use std::fs::File;

pub mod simulation;
pub mod types;

#[tokio::main]
async fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("kanono.log").unwrap()),
        ]
    ).unwrap();

    let arenas = Arc::new(RwLock::new(HashMap::new()));
    arenas.write().await.insert(
        "default".to_string(),
        simulation::Arena::new(
            4000,
            4000
        ),
    );
    let chat = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_connection)
        });

    warp::serve(chat).run(([127, 0, 0, 1], 3000)).await;
}

async fn handle_connection(
    ws: WebSocket,
) {
    info!("New connection: {:?}", ws);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let id: Option<types::Identifer> = None;

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error(uid={}): {}", 0, e);
                break;
            }
        };
        info!("msg: {:?}", msg);
        user_ws_tx.send(msg).await.unwrap();
    }
}