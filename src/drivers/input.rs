use std::io::{event, KeyboardEvent, KeyboardInput, Key, WindowEvent};
use std::collections::HashMap;

pub struct Input {
    pub keys: HashMap<Key, bool>,
}
impl Input {
    pub fn new(context: KeyboardEvent) -> Input {
        Input {
            keys: HashMap::new(),
        }
    }
}
