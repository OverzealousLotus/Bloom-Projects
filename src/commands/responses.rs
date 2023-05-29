// Generic responses to generic messages!

// <=== Standard Library ===>

// <=== Tokio ===>
// use tokio::time::Duration;

// <=== Event Tracking ===>
use tracing::{error, info};

use crate::Context;

// <===== Generic =====>
pub(crate) async fn hate(context: Context<'_>) {
    let dm = {
        context
            .author()
            .dm(&context, |response| response.content("Meanie! Hmph!"))
            .await
    };

    if let Err(reason) = dm {
        error!("Error when direct messaging user: {:?}", reason);
    } else {
        info!("Hmph! {} is a meanie!", context.author().name);
    }
}

async fn speak(emoticon: &str, response: &str, context: Context<'_>) {
    if let Err(reason) = context.say(response).await {
        error!("An error occurred replying: {}", reason);
    } else {
        info!(
            "{} {}'d in channel: {}!",
            context.author().name,
            emoticon,
            context.channel_id()
        );
    }
}

// TODO: Re-Implement generic responses
/*pub async fn generic(context: Context<'_>) {
    match context.author(). {
        "OWO" => speak("owo", "uwu", context).await,

        "UWU" => speak("uwu", "owo", context).await,

        "O3O" => speak("o3o", "u3u", context).await,

        "U3U" => speak("u3u", "o3o", context).await,

        "SERRANA" => speak("hello", "Hi!", context).await,

        _ => {}
    }
}*/
