use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;
use tracing::{error, info};

use crate::Context;

pub(crate) async fn gen_num(limit: usize) -> usize {
    let mut seed = ClockSeed::default(); // Initalize seed variable.

    // Get seed from local clock, and use it to create random number generator.
    let mut random_num = StdRand::seed(seed.next_u64());

    random_num.next_lim_usize(limit) // Grab next usize from generator.
}

pub(crate) async fn speak(response: &str, context: Context<'_>) {
    if let Err(reason) = context.say(response).await {
        error!("An error occurred speaking!: {}", reason)
    } else {
        info!(
            "Speak was invoked for {}!",
            context.command().qualified_name.to_uppercase()
        );
    }
}
