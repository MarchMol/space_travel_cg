
use std::f32::consts::PI;

use nalgebra_glm::{dot, mat4_to_mat3, Mat3, Mat4, Vec2, Vec3, Vec4};
use crate::fragments::{self, Fragment};
use crate::normal_map::{with_normal_map, NormalMap};
use crate::screen::color::{self, Color};
use crate::texture::{self, with_texture, Texture};
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;  
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment :&Fragment, uniforms: &Uniforms)->Color{
  if uniforms.celestial_body.id!="sun"{
    let intensity = calculate_lightning(fragment, uniforms);
    let texture_color = get_fragment_texture(fragment, uniforms);
    texture_color*(intensity.max(0.2).min(2.0))
  } else{
    let texture_color = get_fragment_texture(fragment, uniforms);
    texture_color
  }
}

pub fn get_fragment_texture(fragment: &Fragment, uniforms: &Uniforms)->Color{
  let bid = &uniforms.celestial_body.id;
  let base_color = with_texture(bid,&|texture: &Texture|{
    texture.sample(fragment.texture_pos.x, fragment.texture_pos.y)
  });
  base_color
}

pub fn calculate_lightning(fragment:&Fragment, uniforms: &Uniforms)->f32{
  let bid = &uniforms.celestial_body.id;
  let normal_from_map = with_normal_map(bid,|normal_map: &NormalMap|{
    normal_map.sample(fragment.texture_pos.x, fragment.texture_pos.y)
  });
  let modified_normal = (fragment.normal + normal_from_map).normalize();
  let light_pos = Vec3::new(1.0, 1.0, 0.0);

  dot(&modified_normal, &light_pos)
}