use std::{env, str::FromStr};

use anyhow::{Result, Context};

use crate::{arguments::CommandArgument, game_state::GameState, command_state::CommandState};
mod bot;
mod game_state;
mod order;
mod arguments;
mod command_state;

fn main() -> Result<()> {
    let args = env::args()
        .skip(1)
        .map(|s| CommandArgument::from_str(&s))
        .collect::<Result<Vec<_>>>()
        .context("Unable to parse command line arguments.")?;
    
    let mut command_state = CommandState::new();

    for arg in &args {
        use CommandArgument::*;
        match arg {
            Dance => todo!(),
            CommandArgument::Wait => todo!(),
            CommandArgument::BotId(_) => todo!(),
            CommandArgument::Go => todo!(),
        }
    }

    println!("Command line arguments:");
    for arg in &args {
        println!("{:?}", arg)
    }
    println!();
    let game_state = GameState::new();
    let bot_orders = game_state.get_bot_orders();
    println!("Bot orders:\n{}", bot_orders);
    Ok(())
}
