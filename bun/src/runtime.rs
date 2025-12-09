use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::video::Window;
use std::collections::HashSet;
use std::ffi::{c_void, CStr};
use std::time::{Duration, Instant};

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

    fn update(&mut self, engine: &mut Engine, dt: f32);
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

#[derive(Default)]
pub struct InputState {
    keys_down: HashSet<Keycode>,
}

impl InputState {
    pub fn is_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn pressed_keys(&self) -> impl Iterator<Item = &Keycode> {
        self.keys_down.iter()
    }
}

pub struct Engine {
    window: Window,
    input: InputState,
    aspect_ratio: f32,
    should_close: bool,
}

impl Engine {
    pub(crate) fn new(window: Window, aspect_ratio: f32) -> Self {
        Self {
            window,
            input: InputState::default(),
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

    app.init(&mut engine)?;

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

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        app.update(&mut engine, dt);
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
