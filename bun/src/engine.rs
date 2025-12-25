use glm::Vec2;
use sdl3::event::{Event, WindowEvent};
use sdl3::video::Window;
use crate::input_state::InputState;
use crate::renderer::renderer::Renderer;

pub struct Engine {
    pub(crate) window: Window,
    pub(crate) input: InputState,
    pub(crate) aspect_ratio: f32,
    pub(crate) should_close: bool,
    pub(crate) renderer: Renderer,
}

impl Engine {
    pub(crate) fn new(window: Window, aspect_ratio: f32) -> Self {
        let window_size = window.size();
        let window_size = Vec2::new(window_size.0 as f32, window_size.1 as f32);
        Self {
            window,
            input: InputState::new(window_size),
            aspect_ratio,
            should_close: false,
            renderer: Renderer::new()
        }
    }
    
    pub fn window(&self) -> &Window {
        &self.window
    }
    
    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    
    pub fn input(&self) -> &InputState {
        &self.input
    }
    
    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }
    
    pub fn request_close(&mut self) {
        self.should_close = true;
    }
    
    pub(crate) fn should_close(&self) -> bool {
        self.should_close
    }
    
    pub(crate) fn process_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                self.input.keys_down.insert(*keycode);
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                self.input.keys_down.remove(keycode);
            }
            Event::Window {
                win_event: WindowEvent::Resized(w, h),
                ..
            } => {
                unsafe {
                    gl::Viewport(0, 0, *w, *h);
                }
                self.aspect_ratio = *w as f32 / *h as f32;
                self.input.mouse_state.update_window_size(Vec2::new(*w as f32, *h as f32))
            }
            Event::MouseMotion {
                x, y, xrel, yrel,
                ..
            } => {
                // println!("x: {}, y: {}, xrel: {}, yrel: {}", x, y, xrel, yrel);
                self.input.mouse_state.prev_pos = self.input.mouse_state.pos.clone();
                self.input.mouse_state.pos = Vec2::new(*x, *y);
            }
            _ => {}
        }
    }
}