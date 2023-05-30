// Serrana, a Multi-purpose bot.
#![warn(
    clippy::str_to_string,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    unreachable_pub,
    unused_crate_dependencies
)]
#![forbid(unsafe_code)]

mod assets;
mod commands;

/// Bringing external crates into scope.
use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use tokio::time::Duration;

use tracing::info;

/// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// User data passed to all functions defined.
pub struct Data {
    _votes: DashMap<String, u32>, // Currently unused.
}

/// What to do when an error occurs while running.
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

/// Main function to put everything together.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file!");

    tracing_subscriber::fmt::try_init().expect("Failed to start logger.");

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::meta::help(),
            commands::meta::ping(),
            commands::meta::def_ikaros(),
            commands::meta::count(),
            commands::meta::meter(),
            commands::math::evaluate(),
            commands::owner::coil(),
            commands::owner::dev(),
            commands::games::generic::roshambo(),
            commands::games::generic::preword(),
            commands::games::tycoon::tycoon(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("::".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![
                poise::Prefix::Literal("ser"),
                poise::Prefix::Literal("serrana"),
            ],
            ..Default::default()
        },
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        /// This code is run before every command
        pre_command: |context| {
            Box::pin(async move {
                println!("Executing command {}...", context.command().qualified_name);
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |context| {
            Box::pin(async move {
                info!(
                    "{} used command '{}' in channel {}!",
                    context.author().name,
                    context.command().qualified_name.to_uppercase(),
                    context.channel_id()
                )
            })
        },
        /// Every command invocation must pass this check to continue execution
        command_check: Some(|context| {
            Box::pin(async move {
                if context.author().id == 0 {
                    // Basically bans the user from invoking commands.
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        /// Enforce command checks even for owners (enforced by default)
        /// Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_context, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                // println!("{:?}", _context.data.read().await.get::<_>().unwrap());
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            dotenvy::var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |context, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {
                    _votes: DashMap::new(),
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .initialize_owners(true)
        .run()
        .await
        .unwrap();
}
