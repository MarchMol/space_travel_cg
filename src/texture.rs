use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use nalgebra_glm::Vec3;
use once_cell::sync::{Lazy, OnceCell};
use crate::screen::color::Color;

static TEXTURES: Lazy<Mutex<HashMap<String, Arc<Texture>>>> = Lazy::new(|| Mutex::new(HashMap::new()));



#[derive(Clone, Debug)]
pub struct Texture {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgba8();
        let (width, height) = img.dimensions();
        let data = img.pixels()
            .map(|p| Color::new(p[0] as i32, p[1] as i32, p[2] as i32))
            .collect();

        Ok(Texture {
            width,
            height,
            data,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.fract().abs();
        let v = v.fract().abs();
        
        let x = (u * (self.width as f32)) as u32;
        let y = (v * (self.height as f32)) as u32;
        
        let index = ((self.height-y-1) * self.width + x) as usize;
        self.data[index]
    }
}

// Initialize and store a texture with a given identifier
pub fn init_texture(id: &str, path: &str) -> Result<(), image::ImageError> {
    let texture = Texture::new(path)?;
    let mut textures = TEXTURES.lock().unwrap();
    textures.insert(id.to_string(), Arc::new(texture));
    Ok(())
}

// Retrieve a texture by its identifier and apply a function to it
pub fn with_texture(id: &str, f: impl FnOnce(&Texture) -> Color) -> Color {
    let textures = TEXTURES.lock().unwrap();
    let texture = textures.get(id).expect("Texture not initialized");
    f(texture)
}