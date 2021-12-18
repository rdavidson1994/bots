use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::{game_state::GameState, order::Order};

pub struct CommandState {
    order: Option<Order>,
    bot_ids: HashSet<usize>,
}

impl CommandState {
    pub fn new() -> CommandState {
        CommandState {
            order: None,
            bot_ids: Default::default(),
        }
    }

    pub fn add_bot(&mut self, bot_id: usize) {
        self.bot_ids.insert(bot_id);
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }

    pub fn apply(&self, game_state: &mut GameState) -> Result<()> {
        let order = self.order.context("No order specified.")?;
        for bot_id in &self.bot_ids {
            game_state.edit_bot(*bot_id, |bot| {
                bot.order = order;
            })?
        }
        Ok(())
    }
}
