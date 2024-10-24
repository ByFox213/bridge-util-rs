use std::process::exit;
use futures::StreamExt;
use log::{info, error, debug};
use crate::model::{Env, Msg, MsgHandler};
use crate::patterns::DD_PATTERNS;
use crate::handlers::{rcon_handler};
mod model;
mod patterns;
mod handlers;


#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let env = Env::get_yaml().expect("error load yaml file");
    env_logger::init();

    let nc = match env.connect_nats().await {
        Ok(nc) => {nc}
        Err(err) => {
            eprintln!("Failed connected to nats: {}", err);
            error!("Failed connected to nats: {}", err);
            exit(0)
        }
    };
    let js = async_nats::jetstream::new(nc.clone());

    let mut subscriber = nc.queue_subscribe("tw.econ.read.*", "util".to_string()).await?;

    let commands = env.get_commands();

    info!("Handler started");
    let mut rcon_last = "".to_string();
    while let Some(message) = subscriber.next().await {
        let msg: MsgHandler = match std::str::from_utf8(&message.payload) {
            Ok(json_string) => serde_json::from_str(json_string).unwrap_or_else(|err| {
                error!("Error deserializing JSON: {}", err);
                MsgHandler::default()
            }),
            Err(err) => {
                error!("Error converting bytes to string: {}", err);
                MsgHandler::default()
            }
        };

        if msg.is_default() {
            continue;
        }

        let text_clone = msg.text.clone();
        for pattern in DD_PATTERNS.iter() {
            if !pattern.regex.is_match(&text_clone) {
                continue;
            }

            let caps = pattern.regex.captures(&text_clone).unwrap();
            let value = match pattern.name {
                "rcon" => rcon_handler(caps, &commands).await,
                _ => {continue}
            };


            if value.is_empty() {
                continue;
            }

            let send_msg = Msg {
                server_name: msg.server_name.clone(),
                rcon: value.clone().to_string(),
            };

            let json = match serde_json::to_string_pretty(&send_msg) {
                Ok(str) => {str}
                Err(err) => {error!("Json Serialize Error: {}", err); break}
            };

            if rcon_last == value.clone() {
                continue;
            }

            debug!("send to tw.events: {}", json);
            js.publish("tw.events", json.into())
                .await
                .expect("Error publish message to tw.events");

            debug!("send to tw.econ.moderator: {}", &value);
            js.publish("tw.econ.moderator", value.clone().into())
                .await
                .expect("Error publish message to tw.moderator");

            rcon_last = value;

            break
        }
    }


    Ok(())
}