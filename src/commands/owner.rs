// Restricted Commands

use pyo3::{prelude::*, types::PyModule};

// <=== Event Tracking ===>
use tracing::{error, info};

use crate::{Context, Error};

const CODE: &str = r#"
print("Hello from Rust executing Python code.")
"#;

// <===== Commands =====>
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub(crate) async fn coil(context: Context<'_>) -> Result<(), Error> {
    context.say("*Coils tail and sleeps.*").await?;

    context
        .framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;

    Ok(())
}

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub(crate) async fn dev(context: Context<'_>) -> Result<(), Error> {
    Python::with_gil(|py| {
        let _activators =
            PyModule::from_code(py, CODE, "activators.py", "activators").expect("bruh");
    });

    if let Err(reason) = context.say("Hi").await {
        error!("Error!: {}", reason);
    } else {
        info!("Dev pinged!");
    }
    Ok(())
}
