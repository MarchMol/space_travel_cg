use std::f32::consts::PI;
use std::ops::Add;
use std::os::unix::raw::gid_t;

use nalgebra_glm::{Mat3, Mat4, Vec3, Vec4};
use crate::fragments::Fragment;
use crate::screen::color::{self, Color};
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;

pub fn vertex_shader(
    vertex: &Vertex,
    uniforms: &Uniforms
) -> Vertex{

    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Perspective division
    let w = transformed.w;
    let ndc_position  = Vec4::new(
        transformed.x/w,
        transformed.y/w,
        transformed.z/w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transform normal
  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;
  
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}
