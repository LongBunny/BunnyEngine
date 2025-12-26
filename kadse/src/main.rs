use bun::engine::engine::Engine;
use bun::engine::runtime::{run, App, AppConfig, AppControl, Time};
use bun::glm::{Vec2, Vec4};
use bun::renderer::material::{Material, MaterialProperty, NormalMap};
use bun::renderer::render_object::RenderObject;
use bun::{glm, glm::Vec3, Camera, Event, Keycode, Mesh, One, Shader, Texture, Transform, Zero};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::Arc;
use bun::renderer::mesh_data::MeshData;
use bun::renderer::texture::TextureSpec;

const DEG_TO_RAD: f32 = PI / 180.0;


struct GameState {
    camera: Camera,
    
    bunny: RenderObject,
    cube: RenderObject,
    sphere: RenderObject,
    ground: RenderObject,
    
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
        
        let pbr_shader = Arc::new(Shader::new(
            &PathBuf::from("kadse/res/shaders/pbr.vert"),
            &PathBuf::from("kadse/res/shaders/pbr.frag"),
        )?);
        
        let bunny_texture = Arc::new(Texture::new("kadse/res/textures/gltf_embedded_0.png", TextureSpec::albedo())?);
        let bunny_mat = Arc::new(Material {
            shader: pbr_shader.clone(),
            albedo: MaterialProperty::Texture(bunny_texture.clone()),
            metallic: MaterialProperty::Value(0.0),
            roughness: MaterialProperty::Value(1.0),
            normal: NormalMap::None,
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
        
        
        let cube_mesh = Arc::new(Mesh::from_model(PathBuf::from(
            "kadse/res/models/TestCube/TestCube.obj"
        ))?);
        let sphere_mesh = Arc::new(Mesh::from_model(PathBuf::from(
            "kadse/res/models/sphere.obj"
        ))?);
        // let cube_albedo = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Terracotta/D_Terracotta.jpg", TextureSpec::albedo())?);
        // let cube_metallic = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Terracotta/M_Terracotta.png", TextureSpec::data())?);
        // let cube_normal = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Terracotta/N_Terracotta.jpg", TextureSpec::normal())?);
        // let cube_roughness = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Terracotta/R_Terracotta.png", TextureSpec::data())?);
        
        // let cube_albedo = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_PinkGlass/D_PinkGlass.jpg", TextureSpec::albedo())?);
        // let cube_metallic = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_PinkGlass/M_PinkGlass.jpg", TextureSpec::data())?);
        // let cube_normal = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_PinkGlass/N_PinkGlass.png", TextureSpec::normal())?);
        // let cube_roughness = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_PinkGlass/R_PinkGlass.jpg", TextureSpec::data())?);
        
        // let cube_albedo = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_MetalBubbles/D_MetalBubbles.png", TextureSpec::albedo())?);
        // let cube_metallic = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_MetalBubbles/M_MetalBubbles.png", TextureSpec::data())?);
        // let cube_normal = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_MetalBubbles/N_MetalBubbles.png", TextureSpec::normal())?);
        // let cube_roughness = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_MetalBubbles/R_MetalBubbles.png", TextureSpec::data())?);
        
        let cube_albedo = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Wooden/D_Wooden.png", TextureSpec::albedo())?);
        // let cube_metallic = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Wooden/M_", TextureSpec::data())?);
        let cube_normal = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Wooden/N_Wooden.png", TextureSpec::normal())?);
        let cube_roughness = Arc::new(Texture::new("kadse/res/models/TestCube/Mat_Wooden/R_Wooden.png", TextureSpec::data())?);
        
        let cube_mat = Arc::new(Material {
            shader: pbr_shader.clone(),
            albedo: MaterialProperty::Texture(cube_albedo.clone()),
            // albedo: MaterialProperty::Color(Vec3::new(0.8, 0.589, 0.006)),
            // metallic: MaterialProperty::Texture(cube_metallic.clone()),
            metallic: MaterialProperty::Value(0.0),
            normal: NormalMap::Texture {texture: cube_normal.clone(), scale: 1.0},
            // normal: NormalMap::None,
            roughness: MaterialProperty::Texture(cube_roughness.clone()),
            // roughness: MaterialProperty::Value(0.705),
            ..Default::default()
        });
        let cube = RenderObject::new(
            Transform::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::one() * 5.0,
                Vec3::zero()
            ),
            cube_mesh.clone(),
            cube_mat.clone(),
        );
        
        let sphere_mat = Arc::new(Material {
            shader: pbr_shader.clone(),
            albedo: MaterialProperty::Color(Vec3::one()),
            metallic: MaterialProperty::Value(1.0),
            normal: NormalMap::None,
            roughness: MaterialProperty::Value(0.13),
            ..Default::default()
        });
        let sphere = RenderObject::new(
            Transform::new(
                Vec3::new(2.5, 0.5, 5.0),
                Vec3::one() * 0.75,
                Vec3::zero()
            ),
            sphere_mesh.clone(),
            sphere_mat.clone(),
        );
        
        let ground_albedo = Arc::new(Texture::new("kadse/res/textures/rocky_ground/rocky_terrain_diff_4k.jpg", TextureSpec::albedo())?);
        let ground_normal = Arc::new(Texture::new("kadse/res/textures/rocky_ground/rocky_terrain_nor_gl_4k.png", TextureSpec::normal())?);
        let ground_roughness = Arc::new(Texture::new("kadse/res/textures/rocky_ground/rocky_terrain_rough_4k.png", TextureSpec::data())?);
        let ground_mat = Arc::new(Material {
            shader: pbr_shader.clone(),
            albedo: MaterialProperty::Texture(ground_albedo.clone()),
            normal: NormalMap::Texture {texture: ground_normal.clone(), scale: 1.0},
            roughness: MaterialProperty::Texture(ground_roughness.clone()),
            metallic: MaterialProperty::Value(0.0),
            texture_scale: Vec2::new(10.0, 10.0),
            ..Default::default()
        });
        let ground_mesh = Arc::new(Mesh::from_mesh_data(&MeshData::subdiv_quad(1)));
        let ground = RenderObject::new(
            Transform::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(50.0, 1.0, 50.0),
                Vec3::zero()
            ),
            ground_mesh.clone(),
            ground_mat.clone(),
        );
        
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
            cube,
            sphere,
            ground,
            speed: 7.0,
            rot_speed: 2.0,
            t: 0.0,
        })
    }

    fn reload_shaders(&mut self) {
        println!("Reloading shaders");
        
        match self.cube.material().shader.reload() {
            Ok(_) => println!("pbr shader reloaded!"),
            Err(e) => eprintln!("pbr shader compilation failed: {}", e),
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
                state.speed = (state.speed + (*y * 10.0 * (1.0 / 60.0))).max(0.1);
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
        
        // state
        //     .cube
        //     .transform_mut()
        //     .set_rotation(Vec3::new(rot.x, rot.y + 0.02, rot.z));
        
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
        
        renderer.render(
            &state.cube,
            camera
        );
        
         renderer.render(
            &state.sphere,
            camera
        );
        
        renderer.render(
            &state.ground,
            camera
        );
        
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
