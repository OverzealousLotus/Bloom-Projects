// Meta-Commands

// <=== Tokio ===>
use tokio::time::Duration;

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
| :factory: -> tycoon: Builtin Tycoon!
<=== Math ===>
| :hash: -> evaluate: Evaluate more complex equations.
<=== Definitions ===>
| -> def_ikaros: What is an Ikaros?
<=== Misc ===>
| :exclamation: -> ping: Is Serrana alive?
| :revolving_hearts: -> meter: Love meter between two people!
";

// <===== Functions =====>
async fn gather(prompt: &str, timeout: u64, context: &Context, message: &Message) -> String {
    let _ = message.reply(context, prompt).await;
    let channel = message.channel_id;
    if let Some(answer) = channel
        .await_reply(context)
        .timeout(Duration::from_secs(timeout))
        .await
    {
        answer.content.to_string()
    } else {
        error!("An error occurred trying to fetch intake!");
        String::from("Noop")
    }
}

async fn speak(command: &str, response: &str, context: &Context, message: &Message) {
    if let Err(reason) = message.channel_id.say(&context.http, response).await {
        error!("An error occurred speaking!: {}", reason)
    } else {
        info!("Speak was invoked for {}!", command);
    }
}

async fn assign_num(target: String) -> Result<u64, String> {
    if let Err(reason) = target.parse::<u64>() {
        error!("Error attempting to parse num!: {}", reason);
        Err(String::from("ABORT"))
    } else {
        info!("Successfully parsed to u64!");
        Ok(target.parse::<u64>().unwrap())
    }
}

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
#[aliases(love_metre, metre, lmetre)]
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

#[command]
async fn count(context: &Context, message: &Message) -> CommandResult {
    let mut current_num: u64 = 0;
    let mut next_num: u64 = current_num + 1;
    'mainloop: loop {
        let num =
            assign_num(gather(format!("{}", next_num).as_str(), 60, context, message).await).await;

        next_num += 1;
        if let Err(reason) = num.as_ref() {
            error!("Invalid input!: {}", reason);
            speak("COUNTING", "That's not a number!", context, message).await;
            break 'mainloop;
        }

        if num.as_ref().unwrap() <= &current_num {
            speak("COUNTING", "Streak ruined!", context, message).await;
            info!("Current num: {} User num: {}", current_num, num.unwrap());
            break 'mainloop;
        } else if num.as_ref().unwrap() > &next_num {
            speak("COUNTING", "Streak ruined!", context, message).await;
            info!("Next num: {}, User num: {}", next_num, num.unwrap());
            break 'mainloop;
        } else if num.unwrap() == next_num {
            current_num += 1;
            next_num += 1;
            continue;
        } else {
            speak("COUNTING", "Streak ruined!", context, message).await;
            break 'mainloop;
        }
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
