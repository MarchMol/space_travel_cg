use nalgebra_glm::Vec3;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static NORMAL_MAPS: Lazy<Mutex<HashMap<String, Arc<NormalMap>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
#[derive(Clone, Debug)]
pub struct NormalMap {
    width: u32,
    height: u32,
    data: Vec<Vec3>,
}

impl NormalMap {
    pub fn new(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgba8();
        let (width, height) = img.dimensions();
        
        // Convert RGB colors to normal vectors (assuming tangent-space normal map)
        let data = img.pixels()
            .map(|p| {
                // Convert from [0,255] to [-1,1] range
                let x = (p[0] as f32 / 255.0) * 2.0 - 1.0;
                let y = (p[1] as f32 / 255.0) * 2.0 - 1.0;
                let z = (p[2] as f32 / 255.0) * 2.0 - 1.0;
                Vec3::new(x, y, z).normalize()
            })
            .collect();

        Ok(NormalMap { width, height, data })
    }

    pub fn sample(&self, u: f32, v: f32) -> Vec3 {
        let u = u.fract().abs();
        let v = v.fract().abs();
        
        let x = (u * (self.width as f32)) as u32;
        let y = (v * (self.height as f32)) as u32;
        
        let index = ((self.height-y-1) * self.width + x) as usize;
        self.data[index]
    }
}

pub fn init_normal_map(id: &str,path: &str) -> Result<(), image::ImageError> {
    let normal_map = NormalMap::new(path)?;

    let mut normal_maps = NORMAL_MAPS.lock().unwrap();
    normal_maps.insert(id.to_string(), Arc::new(normal_map));
    Ok(())
}

pub fn with_normal_map(id: &str, f: impl FnOnce(&NormalMap) -> Vec3) -> Vec3 {
    let normal_maps = NORMAL_MAPS.lock().unwrap();
    let normal_map = normal_maps.get(id).expect("Normal map not initialized");
    f(&normal_map)
}