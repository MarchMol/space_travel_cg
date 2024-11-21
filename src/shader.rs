
use nalgebra_glm::{dot, Mat3, Mat4, Vec3, Vec4};
use crate::fragments::{self, Fragment};
use crate::normal_map::{with_normal_map, NormalMap};
use crate::screen::color::{self, Color};
use crate::texture::{self, with_texture, Texture};
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

pub fn fragment_shader(fragment :&Fragment, uniforms: &Uniforms)->Color{
  let intensity = calculate_lightning(fragment, uniforms);
  let texture_color = get_fragment_texture(fragment, uniforms);
  texture_color*(intensity.max(0.2).min(2.0))
}

pub fn get_fragment_texture(fragment: &Fragment, uniforms: &Uniforms)->Color{
  let base_color = with_texture(&|texture: &Texture|{
    texture.sample(fragment.texture_pos.x, fragment.texture_pos.y)
  });
  base_color
}

pub fn calculate_lightning(fragment:&Fragment, uniforms: &Uniforms)->f32{
  let normal_from_map = with_normal_map(|normal_map: &NormalMap|{
    normal_map.sample(fragment.texture_pos.x, fragment.texture_pos.y)
  });
  let modified_normal = (fragment.normal + normal_from_map).normalize();
  dot(&modified_normal, &uniforms.light_dir)
}