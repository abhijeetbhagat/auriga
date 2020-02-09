pub struct Queue {
    routing_key: String
}

impl Queue {
    pub fn new(name: String) -> Self {
        Queue {
            routing_key: String
        }
    }
}