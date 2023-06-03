// Generic games!

// <=== Tokio ===>
use tokio::time::Duration;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Local Assets ===>
use crate::assets::common::{gen_num, speak};
use crate::serenity::MessageBuilder;
use crate::{Context, Error};

// ==========
// Rock, Paper, Scissors!
// ==========

// <===== Constants =====>
const ROCK: &str = "ROCK";
const PAPER: &str = "PAPER";
const SCISSORS: &str = "SCISSORS";

enum State {
    Win,
    Loss,
    Tie,
    Invalid,
}

// <===== Functions =====>
fn rock(player: String) -> State {
    if player == *"PAPER" {
        State::Win
    } else if player == *"SCISSORS" {
        State::Loss
    } else if player == *"ROCK" {
        State::Tie
    } else {
        State::Invalid
    }
}

fn paper(player: String) -> State {
    if player == *"SCISSORS" {
        State::Win
    } else if player == *"ROCK" {
        State::Loss
    } else if player == *"PAPER" {
        State::Tie
    } else {
        State::Invalid
    }
}

fn scissors(player: String) -> State {
    if player == *"ROCK" {
        State::Win
    } else if player == *"PAPER" {
        State::Loss
    } else if player == *"SCISSORS" {
        State::Tie
    } else {
        State::Invalid
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

    speak(&response, context).await;
    let decisions: [&str; 3] = [ROCK, PAPER, SCISSORS];
    let ser_choice = decisions[gen_num(3).await];

    let winner: State = if let Some(answer) = context
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
                State::Tie
            }
        }
    } else {
        State::Tie
    };

    match winner {
        State::Win => {
            let response = MessageBuilder::new()
                .push("You win! I chose: ")
                .push_bold_safe(ser_choice)
                .push("!")
                .build();
            speak(&response, context).await;
        }
        State::Loss => {
            let response = MessageBuilder::new()
                .push("You lost! I chose: ")
                .push_bold_safe(ser_choice)
                .push("!")
                .build();
            speak(&response, context).await;
        }
        State::Tie => {
            let response = MessageBuilder::new()
                .push("It's a tie! I chose: ")
                .push_bold_safe(ser_choice)
                .push("!")
                .build();
            speak(&response, context).await;
        }
        State::Invalid => {
            speak("Error, variable invalid.", context).await;
        }
    }
    Ok(())
}

// ==========
// Guess The Word! Preword!
// ==========

// <===== Constants =====>

// <===== Functions =====>
async fn fetch_word() -> &'static str {
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

    words[gen_num(26).await]
}

// <===== Structs =====>

// <===== Main Logic =====>
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn preword(context: Context<'_>) -> Result<(), Error> {
    info!("{} started a Preword game!", context.author().name);
    let word_one = fetch_word().await;
    let word_two = fetch_word().await;
    let word_three = fetch_word().await;

    let response = MessageBuilder::new()
        .push(word_one)
        .push(", ")
        .push(word_two)
        .push(", or ")
        .push(word_three)
        .push("?")
        .build();

    speak(&response, context).await;
    let decisions: [&str; 3] = [word_one, word_two, word_three];
    let choice = decisions[gen_num(2).await];

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
        speak("Too slow! Game aborted!", context).await;
        error!(
            "Participant failed to reply in channel: {}",
            context.channel_id()
        );
    }

    Ok(())
}
