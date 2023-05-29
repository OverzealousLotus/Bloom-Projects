// Arithmetic

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Mathematics ===>
use fasteval2::{ez_eval, EmptyNamespace};

use crate::{Context, Error};

// <===== Constants =====>
const HELP_MESSAGE: &str = "----------
| Addition: 1 + 1
| Subtraction: 1 - 1
| Multiplication: 2 * 2
| Division: 10 / 2
| Remainder Division: 11 % 2
| Exponentation: 2 ^ 2
| Square Root: sqrt(x)
| Cube Root: cbrt(x)
| <=== Constants ===>
| Pi: pi()
| Tau: tau()
| Phi: phi()
| 
| <=== Comparisons ===>
| Greater than: 10 > 2
| Less than: 2 < 10
| Equal to: 10 == 10
| Not equal to: 10 != 9
| <=== Trigonometry ===>
| Logarithm: log(base, x)
| Natural Log: ln(x)
| Summation: sum(x, y, z...)
| Sine: sin(x)
| Cosine: cos(x)
| Tangent: tan(x)
| Secant: sec(x)
| Hyperbolic Secant: sech(x)
| Cosecant: csc(x)
| Hyperbolic Cosecant: csch(x)
| Cotangent: cot(x)
| Hyperbolic Cotangent: coth(x)
| Integration: integral(x, todo!)";

// <===== Functions =====>

fn expression(input: Vec<f64>) -> f64 {
    let mut ns = EmptyNamespace;
    ez_eval(format!("{:?}", input).as_str(), &mut ns).unwrap()
}
async fn computate(argument: &str, context: Context<'_>) -> Result<(), Error> {
    let mut ns = EmptyNamespace;
    let mut custom_functions = |name: &str, args: Vec<f64>| -> Option<f64> {
        let value = if args.first().is_none() {
            &0.0
        } else {
            args.first().unwrap()
        };
        match name {
            "tau" => Some(std::f64::consts::TAU),

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

            "integral" => {
                let result = expression(args);
                Some(result)
            }
            _ => None,
        }
    };

    let result = ez_eval(argument, &mut custom_functions);
    if let Ok(response) = result {
        info!("{} used evaluate and it succeeded!", context.author().name);
        context.say(response.to_string()).await?;
    } else {
        error!(
            "{} used evaluate! Their equation: {:?} was considered invalid!", // TODO: Re-Implement showing inputted equation.
            context.author().name,
            argument
        );
        context.say("Unable to computate equation...").await?;
    }
    Ok(())
}

// <===== Commands =====>
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn evaluate(context: Context<'_>, args: String) -> Result<(), Error> {
    let argument = args.trim();
    if argument.to_uppercase() != "HELP" {
        computate(argument, context).await?;
    } else if let Err(reason) = context.say(HELP_MESSAGE).await {
        error!("Error printing help message: {}", reason);
    }
    Ok(())
}
