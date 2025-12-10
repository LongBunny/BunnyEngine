use std::collections::HashMap;
use std::iter::Enumerate;
use std::num::NonZeroU32;
use crate::renderer::buffer::{Buffer, EBO, VAO, VBO};
use crate::renderer::vertex::Vertex;
use glm::{IVec3, Vec2, Vec3};
use std::path::Path;
use std::ptr::{null, NonNull};
use std::str::FromStr;
use crate::renderer::mesh_data::MeshData;

#[allow(dead_code)]
pub struct Mesh {
    ebo: EBO,
    vbo: VBO,
    vao: VAO,

    indices_len: usize,
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        let ebo = EBO::new();
        let vbo = VBO::new();
        let vao = VAO::new();

        ebo.bind();
        ebo.buffer_data(&indices);

        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertices);
        vao.vertex_attrib_pointer(0, 3, 8, 0);
        vao.vertex_attrib_pointer(1, 3, 8, 3);
        vao.vertex_attrib_pointer(2, 2, 8, 6);

        ebo.unbind();
        vbo.unbind();
        vao.unbind();

        Self {
            ebo,
            vbo,
            vao,
            indices_len: indices.len(),
        }
    }
    
    pub fn from_mesh_data(mesh_data: &MeshData) -> Self {
        Mesh::new(mesh_data.vertices(), mesh_data.indices())
    }

    pub fn from_model<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let (vertices, indices) = load_from_obj(path)?;
        Ok(Mesh::new(&vertices, &indices))
    }

    pub fn render(&self) {
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_len as i32,
                gl::UNSIGNED_INT,
                null(),
            );
        }
    }
}

fn load_from_obj<P>(path: P) -> Result<(Vec<Vertex>, Vec<u32>), String>
where
    P: AsRef<Path>,
{
    println!("loading obj: {}", path.as_ref().to_string_lossy());
    let content = std::fs::read_to_string(path).unwrap();
    let mut positions: Vec<Vec3> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut uvs: Vec<Vec2> = vec![];
    
    let mut lookup: HashMap<(u32, u32, u32), u32> = HashMap::new();
    let mut faces: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];
    
    for line in content.lines() {
        // comments
        if line.starts_with("#") {
            continue;
        }
        
        let splits: Vec<&str> = line.split(' ').collect();
        let splits = &splits[1..];
        // vertex
        if line.starts_with("v ") {
            positions.push(parse_obj_vector(splits)?);
            continue;
        }
        
        // normal
        if line.starts_with("vn ") {
            normals.push(parse_obj_vector(splits)?);
            continue;
        }
        
        // uv
        if line.starts_with("vt ") {
            let mut uv: Vec2 = parse_obj_vector(splits)?;
            uv.y = 1.0 - uv.y;
            uvs.push(uv);
            continue;
        }
        
        // face
        if line.starts_with("f ") {
            let len = splits.len();
            if len < 3 {
                return Err(String::from("Face has less then 3 vertices"));
            } else if len == 3 {
                // tri
                for split in splits {
                    let parts: IVec3 = parse_obj_vector(&split.split('/').collect::<Vec<&str>>())?;
                    let v_idx = (parts.x - 1) as u32;
                    let vt_idx = (parts.y - 1) as u32;
                    let vn_idx = (parts.z - 1) as u32;
                    
                    let key = (v_idx, vt_idx, vn_idx);
                    let index = if let Some(&i) = lookup.get(&key) {
                        // already has this face
                        i
                    } else {
                        // add new face
                        let vertex = Vertex {
                            v: positions[v_idx as usize],
                            vn: normals[vn_idx as usize],
                            vt: uvs[vt_idx as usize],
                        };
                        
                        let new_index = faces.len() as u32;
                        faces.push(vertex);
                        lookup.insert(key, new_index);
                        
                        new_index
                    };
                    
                    indices.push(index);
                }
                
            } else if len == 4 {
                // quad
                let mut temp_indices: [u32; 4] = [0; 4];
                
                for (i, split) in splits.iter().enumerate() {
                    let parts: IVec3 = parse_obj_vector(&split.split('/').collect::<Vec<&str>>())?;
                    let v_idx = (parts.x - 1) as u32;
                    let vt_idx = (parts.y - 1) as u32;
                    let vn_idx = (parts.z - 1) as u32;
                    
                    let key = (v_idx, vt_idx, vn_idx);
                    let index = if let Some(&i) = lookup.get(&key) {
                        // already has this face
                        i
                    } else {
                        // add new face
                        let vertex = Vertex {
                            v: positions[v_idx as usize],
                            vn: normals[vn_idx as usize],
                            vt: uvs[vt_idx as usize],
                        };
                        
                        let new_index = faces.len() as u32;
                        faces.push(vertex);
                        lookup.insert(key, new_index);
                        
                        new_index
                    };
                    
                    temp_indices[i] = index;
                }
                
                indices.push(temp_indices[0]);
                indices.push(temp_indices[1]);
                indices.push(temp_indices[2]);
                
                indices.push(temp_indices[0]);
                indices.push(temp_indices[2]);
                indices.push(temp_indices[3]);
            } else {
                return Err(String::from("N-Gons are not supported"));
            }
            continue;
        }
        
        println!("Unknown identifier in model: {line}");
    }
    
    Ok((faces, indices))
}


trait FromObjSlice: Sized {
    fn from_slice(parts: &[&str]) -> Result<Self, &'static str>;
}

impl FromObjSlice for Vec2 {
    fn from_slice(parts: &[&str]) -> Result<Self, &'static str> {
        if parts.len() < 2 {
            return Err("Vec 2 needs 2 elements")
        }
        Ok(Vec2 {
            x: f32::from_str(parts[0]).map_err(|_| "bad f32")?,
            y: f32::from_str(parts[1]).map_err(|_| "bad f32")?,
        })
    }
}

impl FromObjSlice for Vec3 {
    fn from_slice(parts: &[&str]) -> Result<Self, &'static str> {
        if parts.len() < 3 {
            return Err("Vec 3 needs 3 elements")
        }
        Ok(Vec3 {
            x: f32::from_str(parts[0]).map_err(|_| "bad f32")?,
            y: f32::from_str(parts[1]).map_err(|_| "bad f32")?,
            z: f32::from_str(parts[2]).map_err(|_| "bad f32")?,
        })
    }
}

impl FromObjSlice for IVec3 {
    fn from_slice(parts: &[&str]) -> Result<Self, &'static str> {
        if parts.len() < 3 {
            return Err("Vec 3 needs 3 elements")
        }
        Ok(IVec3 {
            x: i32::from_str(parts[0]).map_err(|_| "bad i32")?,
            y: i32::from_str(parts[1]).map_err(|_| "bad i32")?,
            z: i32::from_str(parts[2]).map_err(|_| "bad i32")?,
        })
    }
}


fn parse_obj_vector<T: FromObjSlice>(parts: &[&str]) -> Result<T, &'static str> {
    T::from_slice(parts)
}