use crate::fragments::{triangle_fill, Fragment};
use crate::screen::color::Color;
use crate::shader::fragment_shader;
use crate::vertex::Vertex;
use crate::CelestialBody;
use crate::{screen::framebuffer::Framebuffer, shader::vertex_shader};
use nalgebra_glm::{Mat4, Vec3};
pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
    pub celestial_body: CelestialBody,
    pub looking_dir: Vec3
}

impl Uniforms {
    pub fn set_view_matrix(&mut self, view_matrix: &Mat4) {
        self.view_matrix = *view_matrix;
    }
    pub fn increment_time(&mut self) {
        self.time += 1;
    }
    pub fn orbit(&mut self) {
        if self.celestial_body.year > 0.001 && self.celestial_body.day > 0.0 {
            let orbit_speed = self.celestial_body.year * self.time as f32;
            let self_rotation_speed =
                (self.celestial_body.day - self.celestial_body.year) * self.time as f32; // Example for slower self-rotation

            self.celestial_body.translation = Vec3::new(
                self.celestial_body.orbit_radius * orbit_speed.cos(),
                0.0,
                self.celestial_body.orbit_radius * orbit_speed.sin(),
            );
            // Orbit around the origin

            let self_rotation_matrix = Mat4::from_axis_angle(&Vec3::y_axis(), self_rotation_speed);
            let translation_matrix = Mat4::new_translation(&self.celestial_body.translation);
            let scale_matrix = Mat4::new_scaling(self.celestial_body.scale);

            self.model_matrix = translation_matrix * scale_matrix * self_rotation_matrix;
        }
    }
    pub fn translate_model(&mut self, d_translation: &Vec3, d_rotation: &Vec3) {
        self.celestial_body.rotation = self.celestial_body.rotation + d_rotation;
        self.celestial_body.translation = self.celestial_body.translation + d_translation;

        let translation = self.celestial_body.translation;
        let rotation = self.celestial_body.rotation;
        let (sin_x, cos_x) = rotation.x.sin_cos();
        let (sin_y, cos_y) = rotation.y.sin_cos();
        let (sin_z, cos_z) = rotation.z.sin_cos();

        let rotation_matrix_x = Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let rotation_matrix_y = Mat4::new(
            cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let rotation_matrix_z = Mat4::new(
            cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

        let transform_matrix = Mat4::new(
            self.celestial_body.scale,
            0.0,
            0.0,
            translation.x,
            0.0,
            self.celestial_body.scale,
            0.0,
            translation.y,
            0.0,
            0.0,
            self.celestial_body.scale,
            translation.z,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        self.model_matrix = transform_matrix * rotation_matrix;
    }
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // 1. Vertex shader stage
    let mut shaded_vertices: Vec<Vertex> = Vec::new();
    for vertex in vertex_array {
        shaded_vertices.push(vertex_shader(vertex, uniforms))
    }

    // 2. Primitive Assembly stage (only triangles)
    let mut triangles = Vec::new();
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
        fragments.extend(triangle_fill(&tri[0], &tri[1], &tri[2]));
    }
    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        let looking_coeficient = fragment.normal.dot(&uniforms.looking_dir);
        if looking_coeficient > 0.01 {
            let shaded_color = fragment_shader(&fragment, uniforms);
            framebuffer.set_current_color(Color::to_hex(&shaded_color));
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
