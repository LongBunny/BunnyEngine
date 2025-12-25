use bun::engine::engine::Engine;
use bun::engine::runtime::{run, App, AppConfig, AppControl, Time};
use bun::glm::Vec4;
use bun::renderer::material::{Material, MaterialProperty};
use bun::renderer::render_object::RenderObject;
use bun::{glm, glm::Vec3, Camera, Event, Keycode, Mesh, Shader, Texture, Transform};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::Arc;

const DEG_TO_RAD: f32 = PI / 180.0;


struct GameState {
    camera: Camera,
    
    bunny: RenderObject,
    
    speed: f32,
    rot_speed: f32,
    t: f32,
}

impl GameState {
    fn new(engine: &Engine) -> Result<Self, String> {
        let default_shader = Arc::new(Shader::new(
            &PathBuf::from("kadse/res/shaders/default.vert"),
            &PathBuf::from("kadse/res/shaders/default.frag"),
        )?);
        
        let bunny_texture = Arc::new(Texture::new("kadse/res/textures/gltf_embedded_0.png")?);
        let bunny_mat = Arc::new(Material {
            shader: default_shader.clone(),
            albedo: MaterialProperty::Texture(bunny_texture.clone()),
            ..Default::default()
        });
        
        let bunny_mesh = Arc::new(Mesh::from_model(PathBuf::from(
            "kadse/res/models/rabbit.obj",
        ))?);
        
        let bunny_transform = Transform::new(
            Vec3::new(0.0, 3.0, 12.5),
            Vec3::new(40.0, 40.0, 40.0),
            // Vec3::new(0.0, -90f32 * DEG_TO_RAD, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );
        
        let bunny = RenderObject::new(bunny_transform, bunny_mesh.clone(), bunny_mat.clone());
        
        
        
        // let quad_mesh = Rc::new(RefCell::new(Mesh::from_mesh_data(&MeshData::quad())));
        // let cube_mesh = Rc::new(RefCell::new(Mesh::from_model(PathBuf::from(
        //     "kadse/res/models/TestCube/TestCube.obj"
        // ))?));
        
        // let tex_wooden = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_Wooden/D_Wooden.png")?));
        // let tex_terracotta = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_Terracotta/D_Terracotta.jpg")?));
        // let tex_sand = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_Sand/D_StylizedSand.png")?));
        // let tex_pink_glass = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_PinkGlass/D_PinkGlass.jpg")?));
        // let tex_metal_bubbles = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_MetalBubbles/D_MetalBubbles.png")?));
        // let tex_cord_woven = Rc::new(RefCell::new(Texture::new("kadse/res/models/TestCube/Mat_CordWoven/D_CordWoven.png")?));
        // let ground_texture = Texture::new("kadse/res/textures/Grass004_4K-JPG_Color.jpg")?;
        
        // let pbr_models = vec![
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(-7.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_wooden.clone()},
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(-4.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_terracotta.clone()},
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(-1.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_sand.clone()},
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(1.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_pink_glass.clone()},
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(4.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_metal_bubbles.clone()},
        //     PbrModel {model: Model::with_transform(cube_mesh.clone(), default_shader.clone(), Transform::new(Vec3::new(7.5, 0.0, 12.5), Vec3::one() * 10.0, Vec3::zero())), texture: tex_cord_woven.clone()},
        // ];
        
        // let mut subdiv_quad_mesh_data = MeshData::subdiv_quad(32);
        // for vertex in subdiv_quad_mesh_data.vertices_mut() {
        //     vertex.v.y += (fastrand::f32() * 2.0 - 1.0) * 0.3 - 0.2;
        // }
        // let subdiv_quad = Rc::new(RefCell::new(Mesh::from_mesh_data(&subdiv_quad_mesh_data)));
        // let mut floor = Renderable::with_transform(
        //     subdiv_quad.clone(),
        //     default_shader.clone(),
        //     Transform::new(
        //         Vec3::new(0.0, -1.0, 0.0),
        //         Vec3::new(150.0, 1.0, 150.0),
        //         Vec3::new(0.0, 0.0, 0.0),
        //     ),
        // );
        // floor.set_specular_strength(0.0);

        let camera = Camera::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            70.0,
            engine.aspect_ratio(),
            0.01,
            100.0,
        );

        Ok(Self {
            camera,
            bunny,
            speed: 7.0,
            rot_speed: 2.0,
            t: 0.0,
        })
    }

    fn reload_shaders(&mut self) {
        println!("Reloading shaders");

        // self.default_shader.borrow().unbind();
        // match self.default_shader.borrow_mut().reload() {
        //     Ok(_) => println!("default_shader reloaded!"),
        //     Err(e) => eprintln!("default_shader compilation failed: {}", e),
        // }
        //
        // self.checkerboard_shader.borrow().unbind();
        // match self.checkerboard_shader.borrow_mut().reload() {
        //     Ok(_) => println!("checkerboard_shader reloaded!"),
        //     Err(e) => eprintln!("checkerboard_shader compilation failed: {}", e),
        // }
        //
        // self.pbr_shader.borrow().unbind();
        // match self.pbr_shader.borrow_mut().reload() {
        //     Ok(_) => println!("pbr_shader reloaded!"),
        //     Err(e) => eprintln!("pbr_shader compilation failed: {}", e),
        // }
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
            rot.y += self.rot_speed * dt;
            self.camera.set_rotation(rot);
        }
        if input.is_down(Keycode::Right) {
            let mut rot = self.camera.rotation();
            rot.y -= self.rot_speed * dt;
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
        
        // shitty mouse movement
        // {
        //     let mouse_state = input.mouse_state();
        //     let mut rot = self.camera.rotation();
        //     rot.y -= mouse_state.pos().x * self.rot_speed * 15.0 * dt;
        //     rot.x -= mouse_state.pos().y * self.rot_speed * 15.0 * dt;
        //     rot.x = rot.x.min(DEG_TO_RAD * 89.0).max(-DEG_TO_RAD * 89.0);
        //     self.camera.set_rotation(rot);
        // }
        // println!("mouse_state: pos {:?}, prev_pos {:?}", input.mouse_state().pos(), input.mouse_state().prev_pos());
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

    fn update(&mut self, engine: &mut Engine, time: Time) {
        let state = self.state_mut();
        state.camera.set_aspect_ratio(engine.aspect_ratio());
        state.handle_movement(engine, time.dt());
        
        let pos = state.bunny.transform().pos();
        let rot = state.bunny.transform().rotation();
        state
            .bunny
            .transform_mut()
            .set_rotation(Vec3::new(rot.x, rot.y + 0.02, rot.z));
        state
            .bunny
            .transform_mut()
            .set_pos(Vec3::new(pos.x, glm::sin(time.elapsed_secs() * 2.0) + 2.0 + 1.0, pos.z));
        state.t += time.dt();
    }

    fn render(&mut self, engine: &mut Engine) {
        let renderer = engine.renderer();
        renderer.begin_frame(Vec4::new(189.0 / 255.0, 220.0 / 255.0, 237.0 / 255.0, 1.0));
        
        let state = self.state_mut();
        let camera = &state.camera;

        
        renderer.render(
            &state.bunny,
            camera
        );
        
        
        // for pbr_model in state.pbr_models.iter_mut() {
        //     pbr_model.texture.borrow().bind(0).unwrap();
        //     pbr_model.model.render(camera.projection(), camera);
        // }
        //
        // state.ground_texture.bind(0).unwrap();
        // let texture_scale_loc = {
        //     state.floor.shader().get_uniform_location("texture_scale").unwrap()
        // };
        // state.floor.shader().set_uniform(texture_scale_loc, 1.0);
        // state.floor.render(camera.projection(), camera);
        // state.ground_texture.unbind();
        //
        // state.bunny_texture.bind(0).unwrap();
        // let texture_scale_loc = {
        //     state.bunny.shader().get_uniform_location("texture_scale").unwrap()
        // };
        // state.bunny.shader().set_uniform(texture_scale_loc, 1.0);
        // state.bunny.render(camera.projection(), camera);
        // state.bunny_texture.unbind();
        
        
        renderer.end_frame();
        
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
