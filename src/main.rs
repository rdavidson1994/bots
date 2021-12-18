use std::{env, str::FromStr};
use anyhow::{Context, Result};

use crate::{
    arguments::CommandArgument,
    command_state::CommandState,
    game_state::GameState, order::Order,
};
mod arguments;
mod bot;
mod command_state;
mod game_state;
mod order;

fn main() -> Result<()> {
    let args = env::args()
        .skip(1)
        .map(|s| CommandArgument::from_str(&s))
        .collect::<Result<Vec<_>>>()
        .context("Unable to parse command line arguments.")?;

    let mut game_state = GameState::new();
    let mut command_state = CommandState::new();

    for arg in &args {
        use CommandArgument::*;
        match arg {
            Dance => command_state.set_order(Order::Dance),
            Wait => command_state.set_order(Order::Wait),
            BotId(id) => command_state.add_bot(*id),
            Go => command_state
                .apply(&mut game_state)
                .context("Unable to apply commands.")?,
        }
    }

    println!("Command line arguments:");
    for arg in &args {
        println!("{:?}", arg)
    }
    println!();

    let bot_orders = game_state.get_bot_orders();
    println!("Bot orders:\n{}", bot_orders);
    Ok(())
}
