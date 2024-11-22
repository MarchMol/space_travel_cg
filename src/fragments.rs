use core::f32;

use nalgebra_glm::{dot, Vec2, Vec3};
use crate::bounding_box::{barycentric_coordinates, calculate_bounding_box, edge_function};
use crate::screen::color::Color;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub texture_pos: Vec2
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32, normal:Vec3, intensity: f32, texture_pos: Vec2) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal,
            intensity,
            texture_pos,
        }
    }
}

pub fn triangle_fill(v1: &Vertex, v2:&Vertex ,v3:&Vertex, uniforms: &Uniforms)-> Vec<Fragment>{
    let mut fragments = Vec::new();
    let (a,b,c) = (v1.transformed_position,v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let t1 = v1.tex_coords;
    let t2 = v2.tex_coords;
    let t3 = v3.tex_coords;
    let triangle_area = edge_function(&a,&b,&c);
    // Iterate over each pixel in the bounding box
    for y in min_y..max_y{
        for x in min_x..max_x{
            let point = Vec3::new(x as f32+0.5, y as f32+0.5, 0.0);
            
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            let u = t1.x * w1 + t2.x * w2 + t3.x * w3;
            let v = t1.y * w1 + t2.y * w2 + t3.y * w3;

            // if w1!=0.0 || w2!=0.0 || w3!=0.0{
                if w1>=0.0 && w1 <=1.0 &&
                w2>=0.0 && w2 <=1.0 &&
                w3>=0.0 && w3 <=1.0 {
                    let color = Color::new(100, 100, 100);
                    let depth = a.z*w1 +b.z*w2 + c.z*w3;
                    let old_normal = v1.transformed_normal*w1+v2.transformed_normal *w2 + v3.transformed_normal*w3;
                    let normal = old_normal.normalize();
                    fragments.push(
                        Fragment::new(
                            x as f32, 
                            y as f32, 
                            color, 
                            depth, 
                            normal, 
                            0.0,
                            Vec2::new(u,v)
                        )
                    );
                }
            // } 
        }
    }
    fragments
}
