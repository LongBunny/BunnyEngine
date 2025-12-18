use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::video::Window;
use std::collections::HashSet;
use std::ffi::{c_void, CStr};
use std::time::{Duration, Instant};
use glm::Vec2;
use num_traits::Zero;
use sdl3::keyboard::Keycode::Hash;

pub enum AppControl {
    Continue,
    Exit,
}

pub trait App {
    fn init(&mut self, _engine: &mut Engine) -> Result<(), String> {
        Ok(())
    }

    fn handle_event(&mut self, _engine: &mut Engine, _event: &Event) -> AppControl {
        AppControl::Continue
    }

    fn update(&mut self, engine: &mut Engine, dt: Time);
    fn render(&mut self, engine: &mut Engine);
}

#[derive(Clone, Copy)]
pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub max_fps: Option<u32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "bun app",
            max_fps: Some(60),
        }
    }
}

pub struct Time {
    dt: f32,
    elapsed_secs: f32,
}

impl Time {
    pub fn dt(&self) -> f32 { self.dt }
    pub fn elapsed_secs(&self) -> f32 { self.elapsed_secs }
}

pub struct InputState {
    keys_down: HashSet<Keycode>,
    mouse_state: MouseState,
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
    pos: Vec2,
    prev_pos: Vec2,
    
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

pub struct Engine {
    window: Window,
    input: InputState,
    aspect_ratio: f32,
    should_close: bool,
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
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
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

pub fn run<A: App>(config: AppConfig, mut app: A) -> Result<(), String> {
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window(config.title, config.width, config.height)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|e| e.to_string())?;

    unsafe {
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s).unwrap() as *const c_void);
        gl::Viewport(0, 0, config.width as i32, config.height as i32);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(189.0 / 255.0, 220.0 / 255.0, 237.0 / 255.0, 1.0);

        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL version: {}", version.to_string_lossy());
    }

    let mut engine = Engine::new(window, config.width as f32 / config.height as f32);
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    // sdl_context.mouse().relative_mouse_mode(engine.window());
    sdl_context.mouse().warp_mouse_in_window(&engine.window, config.width as f32 / 2.0, config.height as f32 / 2.0);

    app.init(&mut engine)?;

    let mut elapsed_secs = 0.0;
    let mut last_frame = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            if matches!(event, Event::Quit { .. }) {
                engine.request_close();
            }

            engine.process_event(&event);

            if let AppControl::Exit = app.handle_event(&mut engine, &event) {
                engine.request_close();
            }
        }
        
        // sdl_context.mouse().warp_mouse_in_window(&engine.window, config.width as f32 / 2.0, config.height as f32 / 2.0);

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        elapsed_secs += dt;
        last_frame = now;

        app.update(&mut engine, Time { dt, elapsed_secs });
        app.render(&mut engine);
        engine.window.gl_swap_window();

        if engine.should_close() {
            break 'running;
        }

        if let Some(max_fps) = config.max_fps {
            let frame_time = now.elapsed().as_secs_f32();
            let target = 1.0 / max_fps as f32;
            if frame_time < target {
                std::thread::sleep(Duration::from_secs_f32(target - frame_time));
            }
        }
    }

    drop(gl_context);
    Ok(())
}
