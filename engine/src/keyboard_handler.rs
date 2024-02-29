use std::collections::HashMap;

use glfw::{Action, WindowEvent};

pub enum KeyBoardHandlerEvent {
    Pressed(glfw::Key),
    Released(glfw::Key),
}

#[derive(Clone, Debug)]
pub struct KeyBoardHandler {
    pressed: HashMap<glfw::Key, bool>,
}

impl KeyBoardHandler {
    pub fn new() -> Self {
        Self {
            pressed: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: glfw::WindowEvent) -> Option<KeyBoardHandlerEvent> {
        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                self.pressed.insert(key, true);
                Some(KeyBoardHandlerEvent::Pressed(key))
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                self.pressed.remove(&key);
                Some(KeyBoardHandlerEvent::Pressed(key))
            }
            _ => None,
        }
    }
    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        self.pressed.contains_key(&key)
    }
}
