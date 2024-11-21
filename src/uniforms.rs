use crate::fragments::{triangle_fill, Fragment};
use crate::shader::fragment_shader;
use crate::vertex::Vertex;
use crate::{screen::framebuffer::Framebuffer, shader::{vertex_shader}};
use nalgebra_glm::{Mat4, Vec3};
use crate::screen::color::Color;
pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub light_dir: Vec3,
    pub time: u32,
    pub planet: u8
}
pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // 1. Vertex shader stage
    let mut shaded_vertices: Vec<Vertex> = Vec::new();
    for vertex in vertex_array {
        shaded_vertices.push(vertex_shader(vertex, uniforms))
    }

    // 2. Primitive Assembly stage (only triangles)
    let len = shaded_vertices.len();
    let mut triangles= Vec::new();
    
    for i in (0..shaded_vertices.len()).step_by(3) {
        if i + 2 < shaded_vertices.len() {
            triangles.push([
                shaded_vertices[i].clone(),
                shaded_vertices[i + 1].clone(),
                shaded_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage

    let mut fragments: Vec<Fragment> = Vec::new();
    for tri in triangles {
        fragments.extend(triangle_fill(&tri[0], &tri[1], &tri[2], uniforms));
    }
    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        let shaded_color = fragment_shader(&fragment, uniforms);
        framebuffer.set_current_color(Color::to_hex(&shaded_color));
        framebuffer.point(x, y, fragment.depth);
    }
}