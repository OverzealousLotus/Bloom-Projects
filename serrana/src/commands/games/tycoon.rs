// Tycoon

// <=== Standard Library ===>
use std::collections::BTreeMap;
use std::env;

// <=== Tokio ===>
use tokio::fs::{write, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::time::{sleep, Duration};

// <=== Serenity ===>
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Database Parsing ===>
use serde::{Deserialize, Serialize};
use toml;

// <===== Constants =====>
const MAIN_MENU: &str = "What would you like to do?:
Procure: Procure credits by working!
Exit | Quit | Abort: Exit Tycoon!
Todo!: Unimplemented!";

// <===== Functions =====>
async fn intake(prompt: &str, context: &Context, message: &Message) -> String {
    let _ = message.reply(context, prompt).await;
    if let Some(answer) = &message
        .author
        .await_reply(context)
        .timeout(Duration::from_secs(10))
        .await
    {
        answer.content.to_string()
    } else {
        error!("An error occurred trying to fetch intake!");
        String::from("Noop")
    }
}

async fn wait(time: f64) {
    sleep(Duration::from_secs_f64(time)).await;
}

async fn speak(command: &str, response: &str, context: &Context, message: &Message) {
    if let Err(reason) = message.channel_id.say(&context.http, response).await {
        error!("An error occurred speaking!: {}", reason)
    } else {
        info!("Speak was invoked for {}!", command);
    }
}

// <=== Fetch contents of Database ===>
async fn readable(database_path: String) -> String {
    // Prepare to read Database.
    let file = OpenOptions::new()
        .read(true)
        .open(database_path)
        .await
        .expect("Error opening file!");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    // Read Database and store.
    reader
        .read_to_string(&mut contents)
        .await
        .expect("Error reading database!");

    contents
}

async fn produce(
    database_path: String,
    mut amount: usize,
    user: String,
    context: &Context,
    message: &Message,
) -> CommandResult {
    let mut database: Database = toml::from_str(&readable(database_path.clone()).await)
        .expect("Error parsing database to struct!");

    // Increase user credits by specified amount.
    speak("Tycoon: PRODUCE", "Procuring credits...", context, message).await;
    if amount > 50 {
        wait(1.0).await;
        amount -= amount;
        database
            .players
            .entry(user.clone())
            .and_modify(|player| player.credits += amount);
    }

    while amount != 0 {
        wait(0.5).await;
        amount -= 1;
        database
            .players
            .entry(user.clone())
            .and_modify(|player| player.credits += 1);
    }
    let response = MessageBuilder::new()
        .push("Produced credit! New count: ")
        .push_bold_safe(database.players.get(&user).unwrap().credits)
        .build();
    message.channel_id.say(&context.http, response).await?;

    // Save changes to database.
    let new_db = toml::to_string(&database).expect("Error parsing database to TOML!");
    write(database_path, new_db)
        .await
        .expect("Error rewriting database!");

    Ok(())
}

#[command]
#[owners_only]
async fn register_player(context: &Context, message: &Message) -> CommandResult {
    dotenv::dotenv().expect("Error reading environment!");
    let database_path = env::var("DATABASE_PATH").expect("Error reading path to database!");
    let mut database = Database::default();
    let new_user = intake("Enter new user: ", context, message)
        .await
        .trim()
        .to_lowercase();

    database.players.insert(
        new_user.clone(),
        Player {
            username: new_user.to_uppercase(),
            credits: 0,
        },
    );
    let parsed_user = toml::to_string(&database).expect("Error parsing new user to TOML!");

    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .open(database_path)
        .await
        .expect("Failed to fetch path to database!");
    let mut reader = BufReader::new(&mut file);
    let mut contents = String::new();

    reader
        .read_to_string(&mut contents)
        .await
        .expect("Error reading database!");

    if contents.contains(&new_user) {
        error!("{} already exists in database!", &new_user);
    } else {
        info!("Registering {} into database!", &new_user);
        file.write(parsed_user.as_bytes()).await.expect("Foo");
    }
    Ok(())
}

// <===== Structs =====>
#[derive(Serialize, Deserialize, Default, Debug)]
struct Player {
    username: String,
    credits: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Database {
    players: BTreeMap<String, Player>,
}

// <===== Command =====>
#[command]
async fn tycoon(context: &Context, message: &Message) -> CommandResult {
    // Make sure to include a path to your database in an .env file.
    dotenv::dotenv().expect("Error reading environment!");
    info!(
        "Tycoon started by {} in channel: {}!",
        message.author.name, message.channel_id
    );
    let current_user = message.author.name.to_lowercase();
    let database_path: String =
        env::var("DATABASE_PATH").expect("Error fetching path to Database!");

    'main_loop: loop {
        let decision = intake(MAIN_MENU, context, message)
            .await
            .trim()
            .to_uppercase();

        match decision.as_str() {
            "PRODUCE" | "PROCURE" => {
                let time = intake("How long?: ", context, message)
                    .await
                    .parse::<usize>()?;
                produce(
                    database_path.clone(),
                    time,
                    current_user.clone(),
                    context,
                    message,
                )
                .await?;
            }

            "QUIT" | "EXIT" | "ABORT" => {
                speak("EXIT", "Exited tycoon!", context, message).await;
                break 'main_loop;
            }

            "NOOP" => {
                speak("NOOP", "No response, aborting!", context, message).await;
                error!("{} took too long to respond!", message.author.name);
                break 'main_loop;
            }

            _ => {
                speak("INVALID", "Invalid response, retrying!", context, message).await;
            }
        }
    }

    Ok(())
}
