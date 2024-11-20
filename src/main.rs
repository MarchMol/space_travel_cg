use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec1, Vec3};
use screen::framebuffer;
use obj::Obj;
use uniforms::Uniforms;
use std::f32::consts::PI;
use camera::Camera;
use fastnoise_lite::{self, CellularDistanceFunction, DomainWarpType, FastNoiseLite, FractalType, NoiseType};

mod screen;
mod vertex;
mod fragments;
mod obj;
mod uniforms;
mod shader;
mod bounding_box;
mod camera;

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
    let planet =  Obj::load("./assets/3d_models/sphere.obj").expect("Failed to load obj");
    let vertex_array = planet.get_vertex_array();
    let light_dir= Vec3::new(1.0, 3.0, -4.0);
    // Model
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;

    // Camera
    let mut camera = Camera::new(
        Vec3::new(0.0, 1.0, -3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut frame_counter = 0;
    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let mut uniforms = Uniforms { 
      model_matrix: Mat4::identity(), 
      view_matrix: Mat4::identity(),
      projection_matrix, 
      viewport_matrix, 
      light_dir, 
      time: 0, 
      planet: 4
    };
    // Main Window Loop:
    while window.is_open() {
        // Closing listener
        framebuffer.clear();
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Input listener
        handle_input(&window, &mut camera);
        uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        uniforms.time = frame_counter;

        // Rendering stage
        uniforms::render(&mut framebuffer, &uniforms, &vertex_array);
      
        frame_counter+=1;
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


fn create_model_matrix(
    translation: Vec3,
    scale: f32,
    rotation: Vec3
)
->Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();
    
    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );
    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );
    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
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
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    //  camera orbit controls
    if window.is_key_down(Key::Left) {
      camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
      camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
      camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
      camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
      movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
      movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
      movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
      movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
      camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
      camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
      camera.zoom(-zoom_speed);
    }
}