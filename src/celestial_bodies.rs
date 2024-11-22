use std::f32::consts::PI;

use nalgebra_glm::Vec3;

#[derive(Clone, Debug)]
pub struct CelestialBody{
  pub orbit_radius: f32,
  pub translation: Vec3,
  pub scale: f32,
  pub rotation: Vec3,
  pub day: f32,
  pub year: f32,
  pub texture_path: String,
  pub normalmap_path: String,
  pub id: String,
  pub model: usize
}

pub fn init_solar_system()->Vec<CelestialBody>{
  let solar_system = vec![
    // SPACESHIP
    CelestialBody{
      orbit_radius: 30.0,
      translation: Vec3::new(30.0,0.0,0.0),
      scale: 0.05f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: 0.0,
      year: 0.0,
      texture_path: "./assets/textures/spaceship_texture.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/spaceship_np.jpg".to_string(),
      id: "spaceship".to_string(),
      model: 0
    },

    CelestialBody{ // SUN ///
      orbit_radius: 0.0,
      translation: Vec3::new(0.0,0.0,0.0),
      scale: 2.0f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: 0.0,
      year: 0.0,
      texture_path: "./assets/textures/sun.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/moon_np.jpg".to_string(),
      id: "sun".to_string(),
      model: 1
    },

    CelestialBody{ // Mercury //
      orbit_radius: 5.0,
      translation: Vec3::new(5.0,0.0,0.0),
      scale: 0.1f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/80.0),
      year: (PI/200.0),
      texture_path: "./assets/textures/mercury.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/mercury_np.jpg".to_string(),
      id: "mercury".to_string(),
      model: 1
    },
    CelestialBody{ // Venus
      orbit_radius: 6.5,
      translation: Vec3::new(6.5,0.0,0.0),
      scale: 0.4f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/100.0),
      year: (PI/150.0),
      texture_path: "./assets/textures/venus.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/venus_np.jpg".to_string(),
      id: "venus".to_string(),
      model: 1
    },
    CelestialBody{ // Earth
      orbit_radius: 7.6,
      translation: Vec3::new(7.6,0.0,0.0),
      scale: 0.5f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/50.0),
      year: (PI/140.0),
      texture_path: "./assets/textures/earth.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/earth_np.jpg".to_string(),
      id: "earth".to_string(),
      model: 1
    },
    CelestialBody{ // Mars
      orbit_radius: 9.0,
      translation: Vec3::new(9.0,0.0,0.0),
      scale: 0.35f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/170.0),
      year: (PI/230.0),
      texture_path: "./assets/textures/mars.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/mars_np.jpg".to_string(),
      id: "mars".to_string(),
      model: 1
    },
    CelestialBody{ // Jupiter
      orbit_radius: 14.0,
      translation: Vec3::new(14.0,0.0,0.0),
      scale: 1.0f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/200.0),
      year: (PI/200.0),
      texture_path: "./assets/textures/jupiter.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/jupiter_np.jpg".to_string(),
      id: "jupiter".to_string(),
      model: 1
    },
    CelestialBody{ // Saturn
      orbit_radius: 20.0,
      translation: Vec3::new(20.0,0.0,0.0),
      scale: 0.8f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/140.0),
      year: (PI/223.0),
      texture_path: "./assets/textures/saturn.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/saturn_np.jpg".to_string(),
      id: "saturn".to_string(),
      model: 1
    },
    CelestialBody{ // Rings
      orbit_radius: 20.0,
      translation: Vec3::new(20.0,0.0,0.0),
      scale: 1.5f32,
      rotation: Vec3::new(0.3,0.0,0.0),
      day: (PI/140.0),
      year: (PI/223.0),
      texture_path: "./assets/textures/saturns_rings.png".to_string(),
      normalmap_path: "./assets/normal_maps/rings_np.jpg".to_string(),
      id: "rings".to_string(),
      model: 2
    },
    CelestialBody{ // Uranus
      orbit_radius: 25.0,
      translation: Vec3::new(25.0,0.0,0.0),
      scale: 0.6f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/200.0),
      year: (PI/300.0),
      texture_path: "./assets/textures/uranus.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/uranus_np.jpg".to_string(),
      id: "uranus".to_string(),
      model: 1
    },
    CelestialBody{ // Pluto
      orbit_radius: 28.0,
      translation: Vec3::new(28.0,0.0,0.0),
      scale: 0.2f32,
      rotation: Vec3::new(0.0,0.0,0.0),
      day: (PI/200.0),
      year: (PI/200.0),
      texture_path: "./assets/textures/pluto.jpg".to_string(),
      normalmap_path: "./assets/normal_maps/pluto_np.jpg".to_string(),
      id: "pluto".to_string(),
      model: 1
    }
  ];
  solar_system
}