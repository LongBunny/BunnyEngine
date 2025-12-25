use glm::Vec2;
use num_traits::Zero;
use sdl3::keyboard::Keycode;
use std::collections::HashSet;

pub struct InputState {
    pub(crate) keys_down: HashSet<Keycode>,
    pub(crate) mouse_state: MouseState,
}

impl InputState {
    pub fn new(window_size: Vec2) -> Self {
        Self {
            keys_down: HashSet::default(),
            mouse_state: MouseState::new(window_size),
        }
    }
    
    pub fn is_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }
    
    pub fn pressed_keys(&self) -> impl Iterator<Item = &Keycode> {
        self.keys_down.iter()
    }
    
    pub fn mouse_state(&self) -> &MouseState { &self.mouse_state }
}


pub struct MouseState {
    pub(crate) pos: Vec2,
    pub(crate) prev_pos: Vec2,
    
    window_size: Vec2,
}

impl MouseState {
    pub fn new(window_size: Vec2) -> Self {
        Self {
            pos: Vec2::zero(),
            prev_pos: Vec2::zero(),
            window_size
        }
    }
    
    pub fn update_window_size(&mut self, window_size: Vec2) {
        self.window_size = window_size;
    }
    
    pub fn pos(&self) -> Vec2 { (self.pos / self.window_size) * 2.0 - 1.0 }
    pub fn pos_pixel(&self) -> Vec2 { self.pos }
    
    pub fn prev_pos(&self) -> Vec2 { (self.prev_pos / self.window_size) * 2.0 - 1.0 }
    pub fn prev_pos_pixel(&self) -> Vec2 { self.prev_pos }
}