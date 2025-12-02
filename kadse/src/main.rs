use std::cell::RefCell;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::rc::Rc;
use glm::Vec3;
use num_traits::{One, Zero};
use bun::{App, AppSettings, EngineContext, EngineEvent, Keycode, DEG_TO_RAD};
use bun::renderer::camera::Camera;
use bun::renderer::mesh::Mesh;
use bun::renderer::model::{Model, Transform};
use bun::renderer::shader::Shader;
use bun::renderer::texture::Texture;

struct TestGame {
    camera: Option<Camera>,
    texture: Option<Texture>,
    cube1: Option<Model>,
    quad1: Option<Model>,
    floor: Option<Model>,
}

impl TestGame {
    fn new() -> Self {
        Self {
            camera: None,
            texture: None,
            cube1: None,
            quad1: None,
            floor: None,
        }
    }
}

impl App for TestGame {
    fn init(&mut self, ctx: &mut EngineContext) {
        let mut default_shader = Rc::new(RefCell::new(
            Shader::new(&PathBuf::from("kadse/res/shaders/default.vert"), &PathBuf::from("kadse/res/shaders/default.frag")).unwrap()
        ));
        
        let mut checkerboard_shader = Rc::new(RefCell::new(
            Shader::new(&PathBuf::from("kadse/res/shaders/checkerboard.vert"), &PathBuf::from("kadse/res/shaders/checkerboard.frag")).unwrap()
        ));
        
        self.texture = Some(Texture::new("kadse/res/textures/gltf_embedded_0.png").unwrap());
        
        let quad_mesh = Rc::new(RefCell::new(
            Mesh::quad()
        ));
        let cube_mesh = Rc::new(RefCell::new(
            Mesh::cube()
        ));
        let cube_obj_mesh = Rc::new(RefCell::new(
            Mesh::from_model(&PathBuf::from("kadse/res/models/rabbit.obj")).unwrap()
        ));
        
        self.cube1 = Some(Model::with_transform(
            cube_obj_mesh.clone(), default_shader.clone(),
            Transform::new(Vec3::new(0.0, 0.0, 0.0), Vec3::one() * 20.0, Vec3::zero())
        ));
        self.quad1 = Some(Model::with_transform(
            cube_obj_mesh.clone(), default_shader.clone(),
            Transform::new(Vec3::new(5.0, 0.0, -30.0), Vec3::one() * 400.0, Vec3::zero())
        ));
        // let mut cube2 = Model::with_transform(
        //     cube_obj_mesh.clone(), default_shader.clone(),
        //     Transform::new(Vec3::new(-3.0, 0.0, -5.0), Vec3::one(), Vec3::zero())
        // );
        
        self.floor = Some(Model::with_transform(
            quad_mesh.clone(), checkerboard_shader.clone(),
            Transform::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(50.0, 50.0, 1.0), Vec3::new(-std::f32::consts::PI / 2.0, 0.0, 0.0))
        ));
        
        self.camera = Some(Camera::new(
            Vec3::new(0.0, 1.0, 5.0),
            Vec3::new(0.0, -(PI * 0.5), 0.0),
            70.0, ctx.app_settings.aspect_ratio(), 0.01, 100.0
        ));
    
    
        
    }
    
    fn on_event(&mut self, _ctx: &mut EngineContext, _event: &EngineEvent) {
    }
    
    fn update(&mut self, ctx: &mut EngineContext, dt: f32) {
        
        let speed = 4.0;
        let mut rot_speed = 2.0;
        let i = 0f32;
        
        
        let mut texture = self.texture.as_ref().expect("texture not initialized");
        let mut camera = self.camera.as_mut().expect("camera not initialized");
        let mut floor  = self.floor.as_ref().expect("floor not initialized");
        let mut cube1  = self.cube1.as_ref().expect("cube1 not initialized");
        let mut quad1  = self.quad1.as_ref().expect("quad1 not initialized");
        
        
        let old_rot = quad1.transform().rotation();
        quad1.transform_mut().set_rotation(Vec3::new(old_rot.x, old_rot.y + 0.02, old_rot.z));
        
        
        let mut direction = Vec3::zero();
        for keycode in ctx.keycodes.iter() {
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
    }
    
    fn render(&mut self, _ctx: &mut EngineContext) {
        
        let texture = self.texture.as_ref().expect("texture not initialized");
        let camera = self.camera.as_ref().expect("camera not initialized");
        let floor  = self.floor.as_ref().expect("floor not initialized");
        let cube1  = self.cube1.as_ref().expect("cube1 not initialized");
        let quad1  = self.quad1.as_ref().expect("quad1 not initialized");
        
        texture.bind();
        
        floor.render(camera.pv_mat());
        cube1.render(camera.pv_mat());
        // cube2.render(camera.pv_mat());
        quad1.render(camera.pv_mat());
    }
}

fn main() {
    let app = AppSettings::new(1920, 1080, "Hellowo Katse");
    let game = TestGame::new();
    bun::run(game, app);
}