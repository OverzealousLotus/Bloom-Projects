// Generic responses to generic messages!

// <=== Standard Library ===>

// <=== Tokio ===>
// use tokio::time::Duration;

// <=== Serenity ===>
use serenity::model::prelude::*;
use serenity::prelude::*;

// <=== Event Tracking ===>
use tracing::{error, info};

// <===== Generic =====>
pub async fn hate(context: &Context, message: &Message) {
    let dm = {
        message
            .author
            .dm(&context, |response| response.content("Meanie! Hmph!"))
            .await
    };

    if let Err(reason) = dm {
        error!("Error when direct messaging user: {:?}", reason);
    } else {
        info!("Hmph! {} is a meanie!", message.author.name);
    }
}

async fn speak(emoticon: &str, response: &str, context: &Context, message: &Message) {
    if let Err(reason) = message.channel_id.say(&context.http, response).await {
        error!("An error occurred replying: {}", reason);
    } else {
        info!(
            "{} {}'d in channel: {}!",
            message.author.name, emoticon, message.channel_id
        );
    }
}

pub async fn generic(context: &Context, message: &Message) {
    match message.content.to_uppercase().as_str() {
        "OWO" => speak("owo", "uwu", context, message).await,

        "UWU" => speak("uwu", "owo", context, message).await,

        "O3O" => speak("o3o", "u3u", context, message).await,

        "U3U" => speak("u3u", "o3o", context, message).await,

        "SERRANA" => speak("hello", "Hi!", context, message).await,

        _ => {}
    }
}
