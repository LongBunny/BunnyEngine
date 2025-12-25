use crate::engine::Engine;
use sdl3::event::Event;
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
