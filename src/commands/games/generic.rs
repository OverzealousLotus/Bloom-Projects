// Generic games!

// <=== Standard Library ===>

// <=== Tokio ===>
use tokio::time::Duration;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Random ===>
use rand::Rng;

use crate::serenity::MessageBuilder;
use crate::{Context, Error};

// ==========
// Rock, Paper, Scissors!
// ==========

// <===== Constants =====>
const ROCK: &str = "ROCK";
const PAPER: &str = "PAPER";
const SCISSORS: &str = "SCISSORS";

// <===== Functions =====>
fn rock(player: String) -> String {
    if player == *"PAPER" {
        String::from("WIN")
    } else if player == *"SCISSORS" {
        String::from("LOSS")
    } else if player == *"ROCK" {
        String::from("TIE")
    } else {
        String::from("INVALID")
    }
}

fn paper(player: String) -> String {
    if player == *"SCISSORS" {
        String::from("WIN")
    } else if player == *"ROCK" {
        String::from("LOSS")
    } else if player == *"PAPER" {
        String::from("TIE")
    } else {
        String::from("INVALID")
    }
}

fn scissors(player: String) -> String {
    if player == *"ROCK" {
        String::from("WIN")
    } else if player == *"PAPER" {
        String::from("LOSS")
    } else if player == *"SCISSORS" {
        String::from("TIE")
    } else {
        String::from("INVALID")
    }
}

// <===== Structs =====>

// <===== Main Logic =====>
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn roshambo(context: Context<'_>) -> Result<(), Error> {
    info!("{} started a Roshambo game!", context.author().name);

    let response = MessageBuilder::new()
        .push_bold_safe(ROCK)
        .push(", ")
        .push_bold_safe(PAPER)
        .push(", or ")
        .push_bold_safe(SCISSORS)
        .push("?")
        .build();

    let _ = context.say(response).await;
    let decisions: Vec<&'static str> = vec![ROCK, PAPER, SCISSORS];
    let ser_choice = decisions[rand::thread_rng().gen_range(0..3)];

    let winner: String = if let Some(answer) = context
        .author()
        .await_reply(context)
        .timeout(Duration::from_secs(10))
        .await
    {
        match ser_choice {
            "ROCK" => rock(answer.content.clone().to_uppercase()),
            "PAPER" => paper(answer.content.clone().to_uppercase()),
            "SCISSORS" => scissors(answer.content.clone().to_uppercase()),
            _ => {
                error!("Invalid choice!");
                String::from("TIE")
            }
        }
    } else {
        String::from("TIE")
    };

    if winner == "WIN" {
        let response = MessageBuilder::new()
            .push("You win! I chose: ")
            .push_bold_safe(ser_choice)
            .push("!")
            .build();
        let _ = context.say(response).await;
    } else if winner == "LOSS" {
        let response = MessageBuilder::new()
            .push("You lost! I chose: ")
            .push_bold_safe(ser_choice)
            .push("!")
            .build();
        let _ = context.say(response).await;
    } else if winner == "TIE" {
        let response = MessageBuilder::new()
            .push("It's a tie! I chose: ")
            .push_bold_safe(ser_choice)
            .push("!")
            .build();
        let _ = context.say(response).await;
    } else if winner == "INVALID" {
        let _ = context.say("Error, variable invalid.").await;
    } else {
        let _ = context.say("Unknown error!").await;
    }
    Ok(())
}

// ==========
// Guess The Word! Preword!
// ==========

// <===== Constants =====>

// <===== Functions =====>
fn fetch_word() -> &'static str {
    let words: [&'static str; 26] = [
        "AERIAL",
        "BUTTERSCOTCH",
        "CARDS",
        "DOMINO",
        "ECHO",
        "FIREFOX",
        "GECKO",
        "HAUNT",
        "ISAAC",
        "JERRYCAN",
        "KILOMETRE",
        "LEGGING",
        "MICROPHONE",
        "NEWTON",
        "OSCAR",
        "POPSICLE",
        "QUEBEC",
        "ROCK",
        "SANDWICH",
        "TORVALD",
        "UTOPIA",
        "VINCENT",
        "WINTER",
        "XAVIER",
        "YORDLE",
        "ZEALOUS",
    ];

    words[rand::thread_rng().gen_range(0..26)]
}

// <===== Structs =====>

// <===== Main Logic =====>
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn preword(context: Context<'_>) -> Result<(), Error> {
    info!("{} started a Preword game!", context.author().name);
    let word_one = fetch_word();
    let word_two = fetch_word();
    let word_three = fetch_word();

    let response = MessageBuilder::new()
        .push(word_one)
        .push(", ")
        .push(word_two)
        .push(", or ")
        .push(word_three)
        .push("?")
        .build();

    let _ = context.say(response).await;
    let decisions: Vec<&'static str> = vec![word_one, word_two, word_three];
    let choice = decisions[rand::thread_rng().gen_range(0..2)];

    // Await response from participant!
    if let Some(answer) = context
        .author()
        .await_reply(context)
        .timeout(Duration::from_secs(10))
        .await
    {
        if answer.content.to_uppercase().trim() == choice {
            let response = MessageBuilder::new()
                .push(":fireworks:")
                .push_bold_safe("You win! ")
                .push(format!("You chose: {:?}, ", &answer.content.to_uppercase()))
                .push(format!("I chose: {:?}! ", &choice))
                .push(":fireworks:")
                .build();
            let _ = answer.reply(context, response).await;
        } else {
            let response = MessageBuilder::new()
                .push(":sob:")
                .push_bold_safe("You lost! ")
                .push(format!("You chose: {:?}, ", &answer.content.to_uppercase()))
                .push(format!("I chose: {:?} ", &choice))
                .push(":sob:")
                .build();
            let _ = answer.reply(context, response).await;
        }
    } else {
        let _ = context.say("Too slow! Game aborted!").await;
        error!(
            "Participant failed to reply in channel: {}",
            context.channel_id()
        );
    }

    Ok(())
}
