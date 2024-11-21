use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::screen::color::Color;

static TEXTURE: OnceCell<Arc<Texture>> = OnceCell::new();

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

pub fn init_texture(path: &str) -> Result<(), image::ImageError> {
    let texture = Texture::new(path)?;
    TEXTURE.set(Arc::new(texture))
        .expect("Texture already initialized");
    Ok(())
}

pub fn with_texture(f: impl FnOnce(&Texture) -> Color) -> Color {
    let texture = TEXTURE.get()
        .expect("Texture not initialized");
    f(texture)
}