use anyhow::bail;

use crate::bot::Bot;
pub struct GameState {
    bots: Vec<Bot>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            bots: vec![Bot::new(0), Bot::new(1), Bot::new(2)],
        }
    }

    pub fn edit_bot<T>(
        &mut self,
        bot_id: usize,
        edit: impl Fn(&mut Bot) -> T,
    ) -> anyhow::Result<T> {
        if bot_id < self.bots.len() {
            Ok(edit(&mut self.bots[bot_id]))
        }
        else {
            bail!("Invalid bot ID {}", bot_id);
        }
    }

    pub fn get_bot_orders(&self) -> String {
        let mut output = String::new();

        for bot in &self.bots {
            let bot_order_string = format!("{:?}\n", bot.order);
            output.push_str(&bot_order_string)
        }
        output
    }
}
