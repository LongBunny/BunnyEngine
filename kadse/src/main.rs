use bun::{
    gl,
    glm,
    glm::Vec3,
    run, App, AppConfig, AppControl, Camera, Engine, Mesh, Model, Shader, Texture, Transform,
    Event, Keycode,
};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::rc::Rc;

const DEG_TO_RAD: f32 = PI / 180.0;

struct GameState {
    camera: Camera,
    default_shader: Rc<RefCell<Shader>>,
    checkerboard_shader: Rc<RefCell<Shader>>,
    texture: Texture,
    floor: Model,
    bunny: Model,
    speed: f32,
    rot_speed: f32,
    t: f32,
}

impl GameState {
    fn new(engine: &Engine) -> Result<Self, String> {
        let default_shader = Rc::new(RefCell::new(Shader::new(
            &PathBuf::from("kadse/res/shaders/default.vert"),
            &PathBuf::from("kadse/res/shaders/default.frag"),
        )?));

        let checkerboard_shader = Rc::new(RefCell::new(Shader::new(
            &PathBuf::from("kadse/res/shaders/checkerboard.vert"),
            &PathBuf::from("kadse/res/shaders/checkerboard.frag"),
        )?));

        let texture = Texture::new("kadse/res/textures/gltf_embedded_0.png")?;

        let quad_mesh = Rc::new(RefCell::new(Mesh::quad()));
        let bunny_mesh = Rc::new(RefCell::new(Mesh::from_model(PathBuf::from(
            "kadse/res/models/rabbit.obj",
        ))?));

        let bunny = Model::with_transform(
            bunny_mesh.clone(),
            default_shader.clone(),
            Transform::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(40.0, 40.0, 40.0),
                Vec3::new(0.0, -90f32 * DEG_TO_RAD, 0.0),
            ),
        );

        let floor = Model::with_transform(
            quad_mesh.clone(),
            checkerboard_shader.clone(),
            Transform::new(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(50.0, 50.0, 1.0),
                Vec3::new(-PI / 2.0, 0.0, 0.0),
            ),
        );

        let camera = Camera::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, PI * 0.5, 0.0),
            70.0,
            engine.aspect_ratio(),
            0.01,
            100.0,
        );

        Ok(Self {
            camera,
            default_shader,
            checkerboard_shader,
            texture,
            floor,
            bunny,
            speed: 4.0,
            rot_speed: 2.0,
            t: 0.0,
        })
    }

    fn reload_shaders(&mut self) {
        println!("Reloading shaders");

        self.default_shader.borrow().unbind();
        match self.default_shader.borrow_mut().reload() {
            Ok(_) => println!("default_shader reloaded!"),
            Err(e) => eprintln!("default_shader compilation failed: {}", e),
        }

        self.checkerboard_shader.borrow().unbind();
        match self.checkerboard_shader.borrow_mut().reload() {
            Ok(_) => println!("shader_checkerboard reloaded!"),
            Err(e) => eprintln!("shader_checkerboard compilation failed: {}", e),
        }
    }

    fn handle_movement(&mut self, engine: &Engine, dt: f32) {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        let input = engine.input();

        if input.is_down(Keycode::W) {
            direction = direction + self.camera.forward();
        }
        if input.is_down(Keycode::S) {
            direction = direction + self.camera.backward();
        }
        if input.is_down(Keycode::A) {
            direction = direction + self.camera.left();
        }
        if input.is_down(Keycode::D) {
            direction = direction + self.camera.right();
        }
        if input.is_down(Keycode::E) {
            direction = direction + Vec3::new(0.0, 1.0, 0.0);
        }
        if input.is_down(Keycode::Q) {
            direction = direction - Vec3::new(0.0, 1.0, 0.0);
        }

        if input.is_down(Keycode::Left) {
            let mut rot = self.camera.rotation();
            rot.y -= self.rot_speed * dt;
            self.camera.set_rotation(rot);
        }
        if input.is_down(Keycode::Right) {
            let mut rot = self.camera.rotation();
            rot.y += self.rot_speed * dt;
            self.camera.set_rotation(rot);
        }
        if input.is_down(Keycode::Up) {
            let mut rot = self.camera.rotation();
            rot.x = (rot.x - self.rot_speed * dt).max(-DEG_TO_RAD * 89.0);
            self.camera.set_rotation(rot);
        }
        if input.is_down(Keycode::Down) {
            let mut rot = self.camera.rotation();
            rot.x = (rot.x + self.rot_speed * dt).min(DEG_TO_RAD * 89.0);
            self.camera.set_rotation(rot);
        }

        if glm::length(direction) > 0.0 {
            self.camera
                .set_position(self.camera.position() + glm::normalize(direction) * self.speed * dt);
        }
    }
}

struct KadseApp {
    state: Option<GameState>,
}

impl KadseApp {
    fn new() -> Self {
        Self { state: None }
    }

    fn state_mut(&mut self) -> &mut GameState {
        self.state
            .as_mut()
            .expect("app state not initialized before use")
    }
}

impl App for KadseApp {
    fn init(&mut self, engine: &mut Engine) -> Result<(), String> {
        let state = GameState::new(engine)?;
        self.state = Some(state);
        Ok(())
    }

    fn handle_event(&mut self, _engine: &mut Engine, event: &Event) -> AppControl {
        match event {
            Event::Quit { .. } => AppControl::Exit,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => AppControl::Exit,
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                self.state_mut().reload_shaders();
                AppControl::Continue
            }
            Event::MouseWheel { y, .. } => {
                let state = self.state_mut();
                state.speed = (state.speed + (*y as f32 * 10.0 * (1.0 / 60.0))).max(0.1);
                println!("speed: {}", state.speed);
                AppControl::Continue
            }
            _ => AppControl::Continue,
        }
    }

    fn update(&mut self, engine: &mut Engine, dt: f32) {
        let state = self.state_mut();
        state.camera.set_aspect_ratio(engine.aspect_ratio());
        state.handle_movement(engine, dt);

        let rot = state.bunny.transform().rotation();
        state
            .bunny
            .transform_mut()
            .set_rotation(Vec3::new(rot.x, rot.y + 0.02, rot.z));

        state.t += dt;
    }

    fn render(&mut self, engine: &mut Engine) {
        let state = self.state_mut();
        let camera = &state.camera;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        state.texture.bind();
        state.floor.render(camera.projection(), camera.view());
        state.bunny.render(camera.projection(), camera.view());

        if engine.aspect_ratio() > 0.0 {
            state.camera.set_aspect_ratio(engine.aspect_ratio());
        }
    }
}

fn main() -> Result<(), String> {
    let app = KadseApp::new();
    let config = AppConfig {
        width: 1920,
        height: 1080,
        title: "Hellowo Katse",
        max_fps: Some(60),
    };

    run(config, app)
}
