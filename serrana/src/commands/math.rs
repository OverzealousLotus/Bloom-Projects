// Arithmetic

// <=== Serenity ===>
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Mathematics ===>
use fasteval2::{ez_eval, EmptyNamespace};

// <===== Constants =====>
const HELP_MESSAGE: &str = "-----
| Addition: 1 + 1
| Subtraction: 1 - 1
| Multiplication: 2 * 2
| Division: 10 / 2
| Remainder Division: 11 % 2
| Exponentation: 2 ^ 2
| Greater than: 10 > 2
| Less than: 2 < 10
| Equal to: 10 == 10
| Not equal to: 10 != 9";

// <===== Functions =====>
async fn computate(argument: &str, context: &Context, message: &Message) -> CommandResult {
    let mut ns = EmptyNamespace;
    let mut custom_functions = |name: &str, args: Vec<f64>| -> Option<f64> {
        let value = args.get(0).unwrap();
        match name {
            "tau" => Some(ez_eval("pi() * 2", &mut ns).unwrap()),

            "phi" => Some(1.618033988749894),

            "sum" => Some(args.into_iter().sum()),

            "sqrt" => Some(value.sqrt()),

            "cbrt" => Some(value.cbrt()),

            "sec" => Some(1.0 / value.cos()),

            "csc" => Some(1.0 / value.sin()),

            "cot" => Some(value.cos() / value.sin()),

            "sech" => Some(1.0 / value.cosh()),

            "csch" => Some(
                ez_eval(
                    format!("2 / (e()^{:?} - e()^-{:?})", args, args).as_str(),
                    &mut ns,
                )
                .unwrap(),
            ),

            "coth" => Some(value.cosh() / value.sinh()),

            "ln" => Some(ez_eval(format!("log(e(), {:?})", args).as_str(), &mut ns).unwrap()),

            _ => None,
        }
    };

    let result = ez_eval(argument, &mut custom_functions);
    if let Ok(response) = result {
        info!("{} used evaluate and it succeeded!", message.author.name);
        message.channel_id.say(&context.http, response).await?;
    } else {
        error!(
            "{} used evaluate! Their equation: {} was considered invalid!",
            message.author.name, message.content
        );
        message
            .channel_id
            .say(&context.http, "Unable to computate equation...")
            .await?;
    }
    Ok(())
}

// <===== Commands =====>
#[command]
#[aliases(eval)]
async fn evaluate(context: &Context, message: &Message, args: Args) -> CommandResult {
    let argument = args.message().trim();
    if argument.to_uppercase() != "HELP" {
        computate(argument, context, message).await?;
    } else {
        if let Err(reason) = message.channel_id.say(&context.http, HELP_MESSAGE).await {
            error!("Error printing help message: {}", reason);
        }
    }

    Ok(())
}
