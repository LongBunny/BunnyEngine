use glm::{cos, cross, normalize, sin, Mat4, Vec3};
use num_traits::One;

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    
    proj_mat: Mat4,
    view_mat: Mat4,
    pv_mat: Mat4,
}

impl Camera {
    pub fn new(position: Vec3, rotation: Vec3, fov: f32, aspect_ratio: f32, near_clip: f32, far_clip: f32) -> Self {
        let proj_mat = glm::ext::perspective(fov, aspect_ratio, near_clip, far_clip);
        
        let mut result = Self {
            position,
            rotation,
            view_mat: Mat4::one(),
            proj_mat,
            pv_mat: Mat4::one(),
        };
        
        result.calculate_pv_mat();
        
        result
    }
    
    pub fn position(&self) -> Vec3 {
        self.position
    }
    
    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }
    
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.calculate_pv_mat();
    }
    
    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self.calculate_pv_mat();
    }
    
    pub fn view_mat(&self) -> Mat4 {
        self.view_mat
    }
    
    pub fn pv_mat(&self) -> Mat4 {
        self.pv_mat
    }
    
    pub fn forward(&self) -> Vec3 {
        let forward = Vec3::new(
            cos(self.rotation.y) * cos(self.rotation.x),
            sin(self.rotation.x),
            sin(self.rotation.y) * cos(self.rotation.x)
        );
        normalize(forward)
    }
    
    pub fn backward(&self) -> Vec3 {
        -self.forward()
    }
    
    pub fn right(&self) -> Vec3 {
        let forward = self.forward();
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        normalize(cross(forward, world_up))
    }
    
    pub fn left(&self) -> Vec3 {
        -self.right()
    }
    
    pub fn up(&self) -> Vec3 {
        let forward = self.forward();
        let right = self.right();
        normalize(cross(right, forward))
    }
    
    pub fn down(&self) -> Vec3 {
        -self.up()
    }
    
    fn calculate_view_mat(&mut self) {
        self.view_mat = glm::ext::look_at(
            self.position,
            self.position + self.forward(),
            self.up()
        );
    }
    
    fn calculate_pv_mat(&mut self) {
        self.calculate_view_mat();
        self.pv_mat = self.proj_mat * self.view_mat;
    }
}