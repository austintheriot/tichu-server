// #![deny(warnings)]
#![feature(never_type)]
extern crate common;
mod handlers;

use common::{GameState, STCMsg};
use futures::join;
use handlers::{
    index,
    ws::{self, send_message},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::{task, time};
use warp::ws::Message;
use warp::Filter;

use crate::handlers::ws::CLOSE_WEBSOCKET;
#[derive(Debug)]
pub struct UserData {
    is_alive: Arc<RwLock<bool>>,
    tx: mpsc::UnboundedSender<Message>,
    game_id: Option<String>,
}

pub type Users = Arc<RwLock<HashMap<String, UserData>>>;
pub type Games = Arc<RwLock<HashMap<String, GameState>>>;

static PING_INTERVAL_MS: u64 = 60_000;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // universal app state
    let users = Users::default();
    let games = Games::default();

    let users_clone = Arc::clone(&users);
    let games_clone = Arc::clone(&games);

    // send ping messages every 5 messages to every websocket
    let ping_pong = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(PING_INTERVAL_MS));
        loop {
            interval.tick().await;
            for (user_id, ws) in users_clone.read().await.iter() {
                if !*ws.is_alive.read().await {
                    // user didn't respond to ping, close their websocket
                    eprint!("Closing websocket connection for idle user {}", &user_id);
                    ws.tx
                        .send(Message::text(CLOSE_WEBSOCKET))
                        .expect("Couldn't send internal CLOSE websocket message");
                } else {
                    // send ping to user
                    let mut is_alive = ws.is_alive.write().await;
                    *is_alive = false;
                    send_message(
                        user_id.into(),
                        STCMsg::Ping,
                        &Arc::clone(&users_clone),
                        &Arc::clone(&games_clone),
                    )
                    .await;
                }
            }
        }
    });

    // GET /ws -> websocket upgrade
    let ws_route = warp::path("ws")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        // get `user_id` query parameter
        .and(warp::filters::query::raw().map(|e: String| {
            let result = e.split_once('=').expect("Couldn't split string at '='");
            String::from(result.1)
        }))
        // get users hashmap
        .and(warp::any().map(move || Arc::clone(&users)))
        // get games hashmap
        .and(warp::any().map(move || Arc::clone(&games)))
        // combine filters into a handler function
        .map(|ws: warp::ws::Ws, user_id: String, users, games| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| ws::handle_ws_upgrade(socket, user_id, users, games))
        });

    // GET / -> index html
    let index_route = warp::path::end().map(|| warp::reply::html(index::INDEX_HTML));

    let routes = index_route.or(ws_route);

    join!(warp::serve(routes).run(([127, 0, 0, 1], 8001)), ping_pong);
}
