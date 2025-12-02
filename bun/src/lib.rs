use crate::renderer::camera::Camera;
use crate::renderer::mesh::Mesh;
use crate::renderer::model::{Model, Transform};
use crate::renderer::shader::Shader;
use crate::renderer::texture::Texture;
use glm::Vec3;
use num_traits::identities::One;
use num_traits::Zero;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use std::cell::RefCell;
use std::f32::consts::{PI, TAU};
use std::ffi::{c_void, CStr};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use sdl3::Sdl;
use sdl3::video::Window;

mod renderer;

const DEG_TO_RAD: f32 = TAU / 360.0;
const RAD_TO_DEG: f32 = 360.0 / TAU;

pub struct App {
    title: &'static str,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    
    sdl_context: Sdl,
    window: Window,
}

impl App {
    pub fn new(width: u32, height: u32, title: &'static str) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);
        gl_attr.set_double_buffer(true);
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);
        
        let window = video_subsystem.window(title, width, height)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        
        let _gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&_gl_context).unwrap();
        
        let display = window.get_display().unwrap();
        let bounds = display.get_bounds();
        println!("bounds: {bounds:?}");
        
        
        let aspect_ratio = width as f32 / height as f32;
        
        gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s).unwrap() as *const c_void
        });
        
        unsafe {
            let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
            println!("OpenGL version: {}", version.to_string_lossy());
        }
        
        
        Self {
            title,
            width,
            height,
            aspect_ratio,
            
            sdl_context,
            window
        }
    }
}


pub fn test_run() {
    let mut default_shader = Rc::new(RefCell::new(
        Shader::new(&PathBuf::from("/kadse/res/shaders/default.vert"), &PathBuf::from("/kadse/res/shaders/default.frag")).unwrap()
    ));
    
    let mut checkerboard_shader = Rc::new(RefCell::new(
        Shader::new(&PathBuf::from("/kadse/res/shaders/checkerboard.vert"), &PathBuf::from("/kadse/res/shaders/checkerboard.frag")).unwrap()
    ));
    
    let texture = Texture::new("/kadse/res/textures/gltf_embedded_0.png").unwrap();
    
    let quad_mesh = Rc::new(RefCell::new(
        Mesh::quad()
    ));
    let cube_mesh = Rc::new(RefCell::new(
        Mesh::cube()
    ));
    let cube_obj_mesh = Rc::new(RefCell::new(
        Mesh::from_model(&PathBuf::from("/kadse/res/models/rabbit.obj")).unwrap()
    ));
    
    let cube1 = Model::with_transform(
        cube_obj_mesh.clone(), default_shader.clone(),
        Transform::new(Vec3::new(0.0, 0.0, 0.0), Vec3::one() * 20.0, Vec3::zero())
    );
    let mut quad1 = Model::with_transform(
        cube_obj_mesh.clone(), default_shader.clone(),
        Transform::new(Vec3::new(5.0, 0.0, -30.0), Vec3::one() * 400.0, Vec3::zero())
    );
    // let mut cube2 = Model::with_transform(
    //     cube_obj_mesh.clone(), default_shader.clone(),
    //     Transform::new(Vec3::new(-3.0, 0.0, -5.0), Vec3::one(), Vec3::zero())
    // );
    
    let floor = Model::with_transform(
        quad_mesh.clone(), checkerboard_shader.clone(),
        Transform::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(50.0, 50.0, 1.0), Vec3::new(-std::f32::consts::PI / 2.0, 0.0, 0.0))
    );
    
    // floor.set_tint(Vec4::new(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0));
    // quad2.set_tint(Vec4::new(0.0, 1.0, 0.0, 1.0));
    // quad3.set_tint(Vec4::new(0.0, 0.0, 1.0, 1.0));
    
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(0.1, 0.3, 0.2, 1.0);
    }
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 1.0, 5.0),
        Vec3::new(0.0, -(PI * 0.5), 0.0),
        70.0, aspect_ratio, 0.01, 100.0
    );
    
    let mut keycodes: Vec<Keycode> = Vec::new();
    
    let mut speed = 4.0;
    let mut rot_speed = 2.0;
    
    let mut i = 0f32;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let dt = 1.0/60.0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let None = keycodes.iter().find(|kc| **kc == keycode) {
                        keycodes.push(keycode);
                    }
                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::R => {
                            println!("Reloading shaders");
                            default_shader.borrow().unbind();
                            match default_shader.borrow_mut().reload() {
                                Ok(_) => { println!("default_shader reloaded!") }
                                Err(e) => { eprintln!("default_shader compilation failed: {}", e) }
                            }
                            
                            checkerboard_shader.borrow().unbind();
                            match checkerboard_shader.borrow_mut().reload() {
                                Ok(_) => { println!("shader_checkerboard reloaded!") }
                                Err(e) => { eprintln!("shader_checkerboard compilation failed: {}", e) }
                            }
                        }
                        _ => {}
                    }
                },
                Event::KeyUp {keycode: Some(keycode), ..} => {
                    if let Some(index) = keycodes.iter().position(|kc| *kc == keycode) {
                        keycodes.remove(index);
                    }
                }
                Event::MouseWheel {x, y, direction, ..} => {
                    // println!("x: {x}, y: {y}, direction: {direction:?}");
                    speed += y * 10.0 * dt;
                    if speed < 0.1 {
                        speed = 0.1;
                    }
                    
                    println!("speed: {speed}");
                }
                
                Event::Window {win_event: WindowEvent::Resized(w, h), ..} => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                        width = w as u32;
                        height = h as u32;
                        aspect_ratio = w as f32 / h as f32;
                        // TODO: update projection settings
                        // projection = glm::ext::perspective(90f32, aspect_ratio, 0.1, 100.0);
                    }
                }
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        // let old_rot = cube1.transform().rotation();
        // cube1.transform_mut().set_rotation(Vec3::new(old_rot.x + 0.034, old_rot.y + 0.05, old_rot.z - 0.01));
        
        let old_rot = quad1.transform().rotation();
        quad1.transform_mut().set_rotation(Vec3::new(old_rot.x, old_rot.y + 0.02, old_rot.z));
        
        // let old_pos = floor.transform().pos();
        // floor.transform_mut().set_pos(Vec3::new(old_pos.x, old_pos.y - 0.01, old_pos.z));
        
        // quad1.set_tint(Vec4::new(sin(i) * 0.5 + 0.5, 72.0 / 255.0, 213.0 / 255.0, 1.0));
        
        
        // cube2.set_tint(Vec4::new(
        //     sin(i + 1.242 * 0.5) * 0.5 + 0.5,
        //     cos(i + 2.5283 * 0.2) * 0.5 + 0.5,
        //     (sin(i + 0.82 * 0.7) * 0.5 + 0.5 + cos(i + 0.8223 * 1.23) * 0.5 + 0.5) * 0.5,
        //     1.0));
        
        // camera.set_position(camera.position() + Vec3::new(0.01, 0.0, 0.0));
        // camera.set_rotation(camera.rotation() + Vec3::new(0.0, TAU / 10.0 * dt, 0.0));
        
        let mut direction = Vec3::zero();
        for keycode in keycodes.iter() {
            match keycode {
                Keycode::W => {
                    direction = direction + camera.forward();
                }
                Keycode::S => {
                    direction = direction + camera.backward();
                }
                Keycode::A => {
                    direction = direction + camera.left();
                }
                Keycode::D => {
                    direction = direction + camera.right();
                }
                Keycode::Left => {
                    let mut new_rot = camera.rotation();
                    new_rot.y = new_rot.y - rot_speed * dt;
                    camera.set_rotation(new_rot);
                }
                Keycode::Right => {
                    let mut new_rot = camera.rotation();
                    new_rot.y = new_rot.y + rot_speed * dt;
                    camera.set_rotation(new_rot);
                }
                Keycode::Down => {
                    let mut new_rot = camera.rotation();
                    new_rot.x = new_rot.x + rot_speed * dt;
                    if new_rot.x > DEG_TO_RAD * 89.0 {
                        new_rot.x = DEG_TO_RAD * 89.0;
                    }
                    camera.set_rotation(new_rot)
                }
                Keycode::Up => {
                    let mut new_rot = camera.rotation();
                    new_rot.x = new_rot.x - rot_speed * dt;
                    if new_rot.x < -DEG_TO_RAD * 89.0 {
                        new_rot.x = -DEG_TO_RAD * 89.0;
                    }
                    camera.set_rotation(new_rot)
                }
                Keycode::E => {
                    direction = direction + Vec3::new(0.0, 1.0, 0.0);
                }
                Keycode::Q => {
                    direction = direction - Vec3::new(0.0, 1.0, 0.0);
                }
                _ => {}
            }
        }
        if glm::length(direction) > 0.0 {
            camera.set_position(camera.position() + glm::normalize(direction) * speed * dt);
        }
        
        
        texture.bind();
        
        floor.render(camera.pv_mat());
        cube1.render(camera.pv_mat());
        // cube2.render(camera.pv_mat());
        quad1.render(camera.pv_mat());
        
        window.gl_swap_window();
        
        i += 0.01;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}