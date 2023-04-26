// Restricted Commands

// <=== Serenity ===>
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Local Assets ===>
use crate::ShardManagerContainer;

const CODE: &str = r#"
print("Hello from Rust executing Python code.")
"#;

// <===== Commands =====>
#[command]
#[owners_only]
async fn coil(context: &Context, message: &Message) -> CommandResult {
    let data = context.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        message.reply(context, "*Coils tail and sleeps.*").await?;
        info!(
            "{} made me coil in channel: {}!",
            message.author.name, message.channel_id
        );
        manager.lock().await.shutdown_all().await;
    } else {
        message
            .reply(context, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[owners_only]
async fn dev(context: &Context, message: &Message, _args: Args) -> CommandResult {
    Python::with_gil(|py| {
        let activators =
            PyModule::from_code(py, CODE, "activators.py", "activators").expect("bruh");
    });

    if let Err(reason) = message.channel_id.say(&context.http, "Hi").await {
        error!("Error!: {}", reason);
    } else {
        info!("Dev pinged!");
    }
    Ok(())
}
