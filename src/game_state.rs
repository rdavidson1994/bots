use crate::{bot::Bot, order::Order};
pub struct GameState {
    bots: Vec<Bot>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            bots: vec![
                Bot::new(0),
                Bot {
                    order: Order::Dance,
                    id: 1,
                },
                Bot::new(2),
            ],
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
