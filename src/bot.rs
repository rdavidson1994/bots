use crate::order::Order;

pub struct Bot {
    pub id: usize,
    pub order: Order,
}

impl Bot {
    pub fn new(id: usize) -> Bot {
        Bot {
            order: Order::Wait,
            id,
        }
    }
}
