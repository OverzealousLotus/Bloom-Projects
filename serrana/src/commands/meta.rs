// Meta-Commands

// <=== Serenity ===>
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Random ===>
use rand::Rng;

// <===== Constants =====>
const HELP_MESSAGE: &str = "Commands:
| -> :: <-
<=== Minigames ===>
| :regional_indicator_w: -> preword: Word Guessing Game!
| :scissors: -> roshambo: Rock, Paper, Scissors!
<=== Math ===>
| :heavy_plus_sign: -> add: Add two values
| :heavy_minus_sign: -> subtract: Subtract two values
| :heavy_multiplication_x: -> multiply: Multiply two values
| :heavy_division_sign: -> divide: Divide two values
| :hash: -> evaluate: Evaluate more complex equations.
<=== Definitions ===>
| -> def_ikaros: What is an Ikaros?
<=== Misc ===>
| :exclamation: -> ping: Is Serrana alive?
| :revolving_hearts: -> meter: Love meter between two people!
";

// <===== Commands =====>
#[command]
async fn help(context: &Context, message: &Message) -> CommandResult {
    if let Err(reason) = message.channel_id.say(&context.http, &HELP_MESSAGE).await {
        error!("Error sending help message: {:?}", reason);
    }

    Ok(())
}

#[command]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    if let Err(reason) = message.channel_id.say(&context.http, "Pong!").await {
        error!("Error sending message!: {:?}", reason);
    }

    Ok(())
}

#[command]
#[aliases(love_metre, metre)]
async fn meter(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let person_one = args.single::<String>();
    let person_two = args.single::<String>();
    let love_metre = rand::thread_rng().gen_range(0..100);
    let response = MessageBuilder::new()
        .push(":heart: ")
        .push_bold_safe(&person_one?)
        .push(" love for ")
        .push_bold_safe(&person_two?)
        .push(" is at ")
        .push_bold_safe(format!("{:?}%!!! ", &love_metre))
        .push(":heart:")
        .build();

    if let Err(reason) = message.channel_id.say(&context.http, &response).await {
        error!("Error producing love metre: {}", reason);
    }

    info!(
        "{} used meter in channel: {}!",
        message.author.name, message.channel_id
    );
    if let Err(reason) = message.channel_id.say(&context.http, &response).await {
        error!("Error sending debug message!: {}", reason);
    }
    Ok(())
}

// <=====| Definitions |=====>
#[command]
#[aliases(ikaros)]
async fn def_ikaros(context: &Context, message: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push_bold_safe("Ikaros... ")
        .push("An unfortunate empty soul... ")
        .push("lost as a Hollow... ")
        .push("Without reason... Without purpose... ")
        .push("Potential squandered... Without care.")
        .build();
    if let Err(reason) = message.channel_id.say(&context.http, &response).await {
        error!("Error sending message!: {:?}", reason);
    }

    Ok(())
}
