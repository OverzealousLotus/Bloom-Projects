// Arithmetic

// <=== Event Tracking ===>
use tracing::{error, info};

// <=== Mathematics ===>
use fasteval2::{ez_eval, EmptyNamespace};

use crate::assets::common::speak;
use crate::{Context, Error};

// <===== Constants =====>
const HELP_MESSAGE: &str = "
### <===== Basics =====>
- Addition: 1 + 1
- Subtraction: 1 - 1
- Multiplication: 2 * 2
- Division: 10 / 2
- Remainder Division: 11 % 2
- Exponentation: 2 ^ 2
- Square Root: sqrt(x)
- Cube Root: cbrt(x)
- Nth Root: nth_root(n, x)
### <===== Constants =====>
- π: pi()
- τ: tau()
- φ: phi()
- e: e()
### <===== Comparisons =====>
- Greater than: 10 > 2
- Less than: 2 < 10
- Equal to: 10 == 10
- Not equal to: 10 != 9
### <===== Trigonometry =====>
- Logarithm: log(base, x)
   - Natural Log: ln(x)
- Summation: sum(x, y, z...)
- Sine: sin(x)
   - Hyperbolic: sinh()
- Cosine: cos(x)
   - Hyperbolic: cosh()
- Tangent: tan(x)
   - Hyperbolic: tanh()
- Secant: sec(x)
   - Hyperbolic: sech(x)
- Cosecant: csc(x)
   - Hyperbolic: csch(x)
- Cotangent: cot(x)
   - Hyperbolic: coth(x)
- Integration: integral(x, todo!)";

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

            "nth_root" => Some(args.get(1).unwrap().powf(1.0 / args.first().unwrap())),

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
        speak(&format!("{} = {}", argument, response), context).await;
    } else {
        error!(
            "{} used evaluate! Their equation: {:?} was considered invalid!",
            context.author().name,
            argument
        );
        speak("Unable to computate equation...", context).await;
    }
    Ok(())
}

// <===== Commands =====>
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn evaluate(
    context: Context<'_>,
    #[description = "Expression to be evaluated."] args: String,
) -> Result<(), Error> {
    let argument = args.trim();
    if argument.to_uppercase() != "HELP" {
        computate(argument, context).await?;
    } else {
        speak(HELP_MESSAGE, context).await;
    }
    Ok(())
}
