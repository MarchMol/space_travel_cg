use camera::Camera;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3, Vec4};
use normal_map::init_normal_map;
use obj::Obj;
use rand::Rng;
use screen::framebuffer;
use std::f32::consts::PI;
use std::time::Duration;
use texture::init_texture;
use uniforms::Uniforms;

use celestial_bodies::{init_solar_system, CelestialBody};
use loaders::{normal_map, obj, texture};

mod bounding_box;
mod camera;
mod celestial_bodies;
mod fragments;
mod loaders;
mod screen;
mod shader;
mod uniforms;
mod vertex;

fn main() {
    // Window
    let window_width = 800;
    let window_height = 900;
    let mut window = Window::new(
        "3D modeling - Render Pipeline",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_position(500, 500);
    window.update();

    // Framebuffer
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(16);

    // Obj
    // Normal Planet
    let planet = Obj::load("./assets/3d_models/planet.obj").expect("Failed to load obj");
    let vertex_array = planet.get_vertex_array();
    // Spaceship
    let spaceship = Obj::load("./assets/3d_models/spaceship.obj").expect("Failed to load obj");
    let space_vertex_array = spaceship.get_vertex_array();
    // Saturns Rings
    let rings = Obj::load("./assets/3d_models/rings.obj").expect("Failed to load obj");
    let rings_vertex_array = rings.get_vertex_array();

    // Solar system
    let solar_system = init_solar_system();
    let mut uniform_array: Vec<Uniforms> = Vec::new();

    // Testures and Normal Maps init
    for body in &solar_system {
        let path = &body.texture_path;
        let np_path = &body.normalmap_path;
        let id = &body.id;
        init_texture(id, path).expect("Failed to load texture map");
        init_normal_map(id, np_path).expect("Failed to load normal map");
    }
    // Camera
    let mut camera = Camera::new(
        Vec3::new(33.0, 1.5, 0.0),
        Vec3::new(30.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Uniform Array construction
    for body in &solar_system {
        uniform_array.push(Uniforms {
            model_matrix: create_model_matrix(&body.translation, &body.scale, &body.rotation),
            view_matrix: Mat4::identity(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time: 0,
            celestial_body: body.clone(),
            looking_dir: Vec3::zeros(),
        });
    }

    // Star Array
    let stars = generate_stars(framebuffer_width, framebuffer_height, 0.005);

    // Mutable controls
    let mut focus_index = 0;
    let mut birds_eye_view = false;
    let mut focus_mode = false;

    // Main Window Loop:
    while window.is_open() {
        // Clearing framebuffer
        framebuffer.clear();

        // Star printing
        framebuffer.set_current_color(0xffffff);
        for star in &stars {
            framebuffer.point(star.0, star.1, 100.0);
        }

        // Inputs
        if window.is_key_down(Key::Escape) { // Closing
            break;
        }
        planet_selector(&window, &mut focus_mode, &mut focus_index); // Planet selection Listener
        if window.is_key_down(Key::Y) { // Birds-eye View
            birds_eye_view = true;
            birds_view(&mut camera)
        }
        if window.is_key_down(Key::R) { // Reset to ship controns
            focus_mode = false;
            birds_eye_view = false;
            camera.center = uniform_array[0].celestial_body.translation;
            camera.eye = uniform_array[0].celestial_body.translation + Vec3::new(3.0, 1.5, 0.0);
            uniform_array[0].celestial_body.rotation = Vec3::new(0.0, 0.0, 0.0);
        }

        // Camera update
        if focus_mode {
            focus_camera(&mut camera, &mut uniform_array[focus_index]);
        } else if !birds_eye_view {
            move_camera(&window, &mut camera, &mut uniform_array[0]);
        }


        // Main 
        for uni_index in 0..uniform_array.len() { // For body in celestial bodies
            uniform_array[uni_index].looking_dir = camera.eye - camera.center;
            uniform_array[uni_index].set_view_matrix(&create_view_matrix(
                &camera.eye,
                &camera.center,
                &camera.up,
            ));
            uniform_array[uni_index].increment_time();
            uniform_array[uni_index].orbit();
            let is_in_view = is_in_view(
                &uniform_array[uni_index].celestial_body.translation,
                &uniform_array[uni_index].view_matrix,
                &uniform_array[uni_index].projection_matrix,
            );
            if is_in_view {
                let model = uniform_array[uni_index].celestial_body.model;
                if model == 0 {
                    // Spaceship
                    uniforms::render(
                        &mut framebuffer,
                        &uniform_array[uni_index],
                        &space_vertex_array,
                    );
                } else if model == 1 {
                    // Planet
                    let proximity = (camera.center
                        - uniform_array[uni_index].celestial_body.translation)
                        .magnitude();

                    if proximity > uniform_array[uni_index].celestial_body.scale
                        || focus_mode
                        || birds_eye_view
                    {
                        uniforms::render(
                            &mut framebuffer,
                            &uniform_array[uni_index],
                            &vertex_array,
                        );
                    }
                } else {
                    // Rings
                    uniforms::render(
                        &mut framebuffer,
                        &uniform_array[uni_index],
                        &rings_vertex_array,
                    );
                }
            }
        }

        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}

fn is_in_view(position: &Vec3, view_matrix: &Mat4, projection_matrix: &Mat4) -> bool {
  let pos4 = Vec4::new(position.x, position.y, position.z, 1.0);

  // Transform to clip space
  let view_position = view_matrix * pos4;
  let clip_position = projection_matrix * view_position;

  // Perform the homogeneous division
  if clip_position.w == 0.0 {
      return false; // Avoid division by zero
  }

  let ndc_position = clip_position / clip_position.w;

  // Check if it's within the normalized device coordinates range
  ndc_position.x >= -1.0
      && ndc_position.x <= 1.0
      && ndc_position.y >= -1.0
      && ndc_position.y <= 1.0
      && ndc_position.z >= -1.0
      && ndc_position.z <= 1.0
}

fn generate_stars(width: usize, height: usize, density: f32) -> Vec<(usize, usize)> {
    // Calculate the total number of stars based on the density
    let total_area = (width * height) as f32;
    let number_of_stars = (density * total_area) as usize;

    // Initialize a random number generator
    let mut rng = rand::thread_rng();

    // Generate random positions for each star
    let mut stars = Vec::with_capacity(number_of_stars);
    for _ in 0..number_of_stars {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        stars.push((x, y));
    }

    stars
}

// Camera Focus
fn birds_view(camera: &mut Camera) {
    camera.center = Vec3::new(0.0, 0.0, 0.0);
    camera.eye = Vec3::new(40.0, 40.0, 0.0);
}
fn focus_camera(camera: &mut Camera, uniform: &mut Uniforms) {
    let inverse_scale = uniform.celestial_body.scale;
    let direction = (camera.eye - camera.center).normalize();
    camera.center = uniform.celestial_body.translation;
    camera.eye = uniform.celestial_body.translation
        + Vec3::new(
            (direction.x + 5.0) * inverse_scale,
            1.5 * inverse_scale,
            direction.z * inverse_scale,
        );
}

// Matrix generation
fn create_model_matrix(translation: &Vec3, scale: &f32, rotation: &Vec3) -> Mat4 {
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
        *scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        *scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        *scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
    look_at(eye, center, up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

// Inputs
fn move_camera(window: &Window, camera: &mut Camera, uniform: &mut Uniforms) {
  let rotation_speed = PI / 25.0;
  let translation_speed = 0.2;
  let zoom_speed = 0.1;
  let direction = (camera.eye - camera.center).normalize();
  let forward = Vec3::new(direction.x, 0.0, direction.z) * translation_speed;

  if window.is_key_down(Key::A) {
      camera.orbit(-rotation_speed, 0.0);
      uniform.translate_model(
          &Vec3::new(0.0, 0.0, 0.0),
          &Vec3::new(0.01, rotation_speed, 0.0),
      );
  }
  if window.is_key_down(Key::D) {
      camera.orbit(rotation_speed, 0.0);
      uniform.translate_model(
          &Vec3::new(0.0, 0.0, 0.0),
          &Vec3::new(-0.01, -rotation_speed, 0.0),
      );
  }

  if window.is_key_down(Key::W) {
      uniform.translate_model(&-forward, &Vec3::new(0.0, 0.0, 0.0));
      camera.center = uniform.celestial_body.translation;
      camera.eye = camera.eye - forward;
  }
  if window.is_key_down(Key::S) {
      uniform.translate_model(&forward, &Vec3::new(0.0, 0.0, 0.0));
      camera.center = uniform.celestial_body.translation;
      camera.eye = camera.eye + forward;
  }

  if window.is_key_down(Key::Q) {
      // Zoom out
      camera.zoom(-zoom_speed);
  }
  if window.is_key_down(Key::E) {
      // Zoom In
      camera.zoom(zoom_speed);
  }
}


fn planet_selector(window: &Window, focus_mode: &mut bool, focus_index: &mut usize) {
  if window.is_key_down(Key::Key1) {
      *focus_index = 2;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key2) {
      *focus_index = 3;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key3) {
      *focus_index = 4;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key4) {
      *focus_index = 5;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key5) {
      *focus_index = 6;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key6) {
      *focus_index = 7;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key7) {
      *focus_index = 9;
      *focus_mode = true;
  }
  if window.is_key_down(Key::Key8) {
      *focus_index = 10;
      *focus_mode = true;
  }
}