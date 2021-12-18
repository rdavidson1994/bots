use std::{collections::HashSet, future::Ready};

use anyhow::Result;

use crate::{arguments::CommandArgument, order::Order, game_state::GameState};

pub struct CommandState {
    order: Option<Order>,
    bot_ids: HashSet<usize>
}

impl CommandState {
    pub fn new() -> CommandState {
        CommandState {
            order: None,
            bot_ids: Default::default()
        }
    }

    pub fn add_bot(&mut self, bot_id: usize) {
        self.bot_ids.insert(bot_id);
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }

    pub fn apply(&self, game_state: &mut GameState) -> Result<()> {
        
        Ok(())
    }
}
