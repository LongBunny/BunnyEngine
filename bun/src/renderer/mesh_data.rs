use crate::renderer::vertex::VertexLayout;
use crate::Vertex;
use glm::{Vec2, Vec3};
use std::marker::PhantomData;

pub struct MeshData<V: VertexLayout> {
    vertices: Vec<V>,
    indices: Vec<u32>,
    _marker: PhantomData<V>
}

impl<V: VertexLayout> MeshData<V> {
    pub fn new(vertices: Vec<V>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            _marker: PhantomData
        }
    }

    pub fn vertices(&self) -> &Vec<V> {
        &self.vertices
    }
    
    pub fn vertices_mut(&mut self) -> &mut Vec<V> {
        self.vertices.as_mut()
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

impl MeshData<Vertex> {
    pub fn quad() -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex {
                v: Vec3::new(-0.5, 0.0, 0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.0, -0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.0, -0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.0, 0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(1.0, 1.0),
            },
        ];
        
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
        
        Self { vertices, indices, _marker: PhantomData }
    }
    
    pub fn subdiv_quad(res: u32) -> Self {
        let mut vertices = vec![];
        let mut indices = vec![];
        
        let spacing = 1.0 / res as f32;
        let normal = Vec3::new(0.0, 1.0, 0.0);
        for row in 0..=res {
            for col in 0..=res {
                let x_advance = spacing * col as f32;
                let z_advance = spacing * row as f32;
                let x = -0.5 + x_advance;
                let y = 0.0f32;
                let z = 0.5 - z_advance;
                
                let u = 0.0 + x_advance;
                let v = 1.0 - z_advance;
                vertices.push(Vertex::new(Vec3::new(x, y, z), normal, Vec2::new(u, v)));
            }
        }
        
        for row in 0..res {
            for col in 0..res {
                let stride = res + 1;
                indices.push(col + row * stride);
                indices.push((col + 1) + row * stride);
                indices.push(col + (row + 1) * stride);
                
                indices.push(col + (row + 1) * stride);
                indices.push((col + 1) + row * stride);
                indices.push((col + 1) + (row + 1) * stride);
            }
        }
        
        Self { vertices, indices, _marker: PhantomData }
    }
    
    pub fn cube() -> Self {
        let vertices: Vec<Vertex> = vec![
            // Front face (+Z) - Red
            Vertex {
                v: Vec3::new(-0.5, -0.5, 0.5),
                vn: Vec3::new(0.0, 0.0, 1.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, -0.5, 0.5),
                vn: Vec3::new(0.0, 0.0, 1.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, 0.5),
                vn: Vec3::new(0.0, 0.0, 1.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.5, 0.5),
                vn: Vec3::new(0.0, 0.0, 1.0),
                vt: Vec2::new(0.0, 1.0),
            },
            // Back face (-Z) - Green
            Vertex {
                v: Vec3::new(0.5, -0.5, -0.5),
                vn: Vec3::new(0.0, 0.0, -1.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(-0.5, -0.5, -0.5),
                vn: Vec3::new(0.0, 0.0, -1.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.5, -0.5),
                vn: Vec3::new(0.0, 0.0, -1.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, -0.5),
                vn: Vec3::new(0.0, 0.0, -1.0),
                vt: Vec2::new(0.0, 1.0),
            },
            // Top face (+Y) - Blue
            Vertex {
                v: Vec3::new(-0.5, 0.5, 0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, 0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, -0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.5, -0.5),
                vn: Vec3::new(0.0, 1.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
            // Bottom face (-Y) - Yellow
            Vertex {
                v: Vec3::new(-0.5, -0.5, -0.5),
                vn: Vec3::new(0.0, -1.0, 0.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, -0.5, -0.5),
                vn: Vec3::new(0.0, -1.0, 0.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, -0.5, 0.5),
                vn: Vec3::new(0.0, -1.0, 0.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(-0.5, -0.5, 0.5),
                vn: Vec3::new(0.0, -1.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
            // Right face (+X) - Magenta
            Vertex {
                v: Vec3::new(0.5, -0.5, 0.5),
                vn: Vec3::new(1.0, 0.0, 0.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, -0.5, -0.5),
                vn: Vec3::new(1.0, 0.0, 0.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, -0.5),
                vn: Vec3::new(1.0, 0.0, 0.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(0.5, 0.5, 0.5),
                vn: Vec3::new(1.0, 0.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
            // Left face (-X) - Cyan
            Vertex {
                v: Vec3::new(-0.5, -0.5, -0.5),
                vn: Vec3::new(-1.0, 0.0, 0.0),
                vt: Vec2::new(0.0, 0.0),
            },
            Vertex {
                v: Vec3::new(-0.5, -0.5, 0.5),
                vn: Vec3::new(-1.0, 0.0, 0.0),
                vt: Vec2::new(1.0, 0.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.5, 0.5),
                vn: Vec3::new(-1.0, 0.0, 0.0),
                vt: Vec2::new(1.0, 1.0),
            },
            Vertex {
                v: Vec3::new(-0.5, 0.5, -0.5),
                vn: Vec3::new(-1.0, 0.0, 0.0),
                vt: Vec2::new(0.0, 1.0),
            },
        ];
        
        let indices: Vec<u32> = vec![
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Top
            8, 9, 10, 10, 11, 8, // Bottom
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Left
            20, 21, 22, 22, 23, 20,
        ];
        
        Self { vertices, indices, _marker: PhantomData }
    }
    
    pub fn cube_sphere() -> Self {
        let vertices = vec![];
        let indices = vec![];
        
        Self { vertices, indices, _marker: PhantomData }
    }
}
