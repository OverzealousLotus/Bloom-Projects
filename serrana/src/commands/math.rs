// Arithmetic

// <=== Serenity ===>
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Mathematics ===>
use evalexpr::eval;

// <===== Commands =====>
#[command]
#[aliases(eval)]
async fn evaluate(context: &Context, message: &Message, args: Args) -> CommandResult {
    let result = eval(args.message().trim());

    if let Ok(response) = result {
        info!("{} used evaluate and it succeeded!", message.author.name);
        message.channel_id.say(&context.http, response).await?;
    } else {
        error!(
            "{} used evaluate()! Their equation: {} was considered invalid!",
            message.author.name,
            args.message()
        );
        message
            .channel_id
            .say(&context.http, "Invalid equation.")
            .await?;
    }

    Ok(())
}
