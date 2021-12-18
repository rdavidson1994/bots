use crate::{arguments::CommandArgument, order::Order};

pub struct CommandState {
    order: Option<Order>,
    bot_ids: Vec<usize>
}

impl CommandState {
    pub fn new() -> CommandState {
        CommandState {
            order: None,
            bot_ids: vec![]
        }
    }
}
