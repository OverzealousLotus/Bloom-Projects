// Serrana, a Multi-Purpose Discord Bot

// <=== Modules ===>
mod commands;

// <=== Standard Library ===>
use std::collections::HashSet;
use std::env;
use std::sync::Arc;

// <=== Serenity ===>
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Local Assets ===>
use crate::commands::games::{generic::*, tycoon::*};
use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::owner::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        match message.content.to_uppercase().as_str() {
            "I LOVE YOU SERRANA" | "ILY SERRANA" => {
                let response = MessageBuilder::new()
                    .push("I love you too, ")
                    .push_bold_safe(&message.author.name)
                    .build();

                if let Err(reason) = message.channel_id.say(&context.http, &response).await {
                    error!("Error sending message: {:?}", reason);
                }
            }

            "I HATE YOU SERRANA" => {
                commands::responses::hate(&context, &message).await;
            }

            "UWU" | "OWO" | "O3O" | "U3U" if !message.author.bot => {
                commands::responses::generic(&context, &message).await;
            }

            _ => {} // Do nothing!
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(evaluate, ping, meter, help, preword, roshambo, tycoon, def_ikaros)]
struct General;

#[group]
#[commands(dev, coil, register_player)]
struct Owner;

#[tokio::main]
async fn main() {
    // Initialize Environment Variables in .env file.
    dotenvy::dotenv().expect("Failed to load .env file!");

    // Initialize the logger to use environment variables.
    tracing_subscriber::fmt::init();

    // If you're going to use this codebase, make sure to set token to your bot's token.
    // Otherwise, your version of Serrana won't work.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    // Fetch bot owners & id.
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(reason) => panic!("Could not access application info: {:?}", reason),
    };

    // Create framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("::").case_insensitivity(true))
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(reason) = client.start().await {
        error!("Client error: {:?}", reason);
    }
}
