use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::{screen::color::Color, vertex::Vertex};

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    pub fn get_vertex_array(&self)-> Vec<Vertex>{
        let mut vertex_array = Vec::new();
        let vertex_color = Color::from_hex(0x5797ff);
        for i in &self.indices{
            vertex_array.push(
                Vertex{
                    color: vertex_color,
                    position: self.vertices[*i as usize],
                    normal: self.normals[*i as usize],
                    tex_coords: self.texcoords[*i as usize],
                    transformed_normal: Vec3::new(0.0,0.0,0.0),
                    transformed_position: Vec3::new(0.0,0.0,0.0),
                }
            )
        }
        vertex_array
    }
}