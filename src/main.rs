mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hitlist;
mod material;
mod camera;
mod scene;
mod raytrace;
mod aabb;
mod bvh;

use std::sync::{Arc, Mutex};
use glium::*;
use std::time::{Duration, Instant};
use glium::glutin::dpi::PhysicalSize;
use glium::texture::RawImage2d;
use image::{RgbaImage};
use crate::hitlist::HitList;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use crate::camera::Camera;
use crate::material::{Materials};
use crate::raytrace::RTParams;

extern crate glium;
extern crate image;

#[derive(Copy, Clone)]
struct TexVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(TexVertex, position, tex_coords);

static ASPECT_RATIO: f64 = 16.0 / 9.0;
static IMAGE_WIDTH: u32 = 1280;
static IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
static SAMPLES_PER_PIXEL: u32 = 10;
static MAX_DEPTH: i32 = 10;

fn main() {
    // Create our image object and wrap it within Arc<Mutex>
    let image = RgbaImage::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let shared_image = Arc::new(Mutex::new(image));

    // Initialize window and opengl context
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new();
    wb = wb.with_inner_size(PhysicalSize { width: IMAGE_WIDTH, height: IMAGE_HEIGHT });
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Create square shape to pass to GPU
    let vertex1 = TexVertex { position: [-1.0, 1.0], tex_coords: [0.0, 1.0] };
    let vertex2 = TexVertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] };
    let vertex3 = TexVertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };
    let vertex4 = TexVertex { position: [1.0, -1.0], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    // Create vertex buffer for shape
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Create index buffer
    let indices: [u16; 6] = [0, 1, 2, 1, 2, 3];
    let index_buffer = glium::IndexBuffer::new(&display,
                                               glium::index::PrimitiveType::TrianglesList,
                                               &indices).unwrap();

    // Compile the shader program
    let vertex_shader_src = include_str!("shaders/vert.vs");
    let fragment_shader_src = include_str!("shaders/frag.fs");
    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None).unwrap();

    // Create another handle to the image and run the RT on another thread
    let image_copy = shared_image.clone();
    std::thread::spawn(move || {
        raytrace::run_rt(RTParams::new(
                ASPECT_RATIO,
                IMAGE_WIDTH,
                SAMPLES_PER_PIXEL,
                MAX_DEPTH),
                         &scene::random_scene(), image_copy);
    });

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let image = shared_image.lock().unwrap().clone();
        let dimensions = image.dimensions();
        let gpu_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&display, gpu_image).unwrap();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program,
                    &uniform! { tex: &texture },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = Instant::now() + Duration::from_millis(16);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
