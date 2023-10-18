mod shader;
mod camera;
mod game_window;
mod cube;
mod texture;
mod game_specs;
mod world;
mod renderer;

extern crate gl;
use gl::types::*;

extern crate glutin;
use glutin::event_loop::{ControlFlow, EventLoop};

extern crate cgmath;
use cgmath::{Deg, InnerSpace, Matrix, Matrix4, perspective, SquareMatrix, vec3, Vector3};

use game_window::GameWindow;
use crate::game_specs::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::renderer::Renderer;
use crate::world::World;

fn main() {

    // Initialize the event loop and window builder
    //the event loop handles events such as keyboard and mouse input, window resizing, and more.
    let event_loop = EventLoop::new();
    let mut window = GameWindow::new();

    // Initialize OpenGL (make opengl functions available within the program)
    // TODO this should get moved I'm just not sure where
    gl::load_with(|symbol| window.context.get_proc_address(symbol) as *const _);

    let world = World::new();
    let test_cube_pos = Vector3::new(0.0, 0.0, 0.0);

    let mut renderer = Renderer::new();
    renderer.init_renderer(world);

    // Initialize variables for tracking time
    let mut last_frame_time = std::time::Instant::now();
    let mut delta_time = std::time::Duration::new(0, 0);

    // Main event loop runs until application is terminated.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        //calculate time between frames
        let current_frame_time = std::time::Instant::now();
        delta_time = current_frame_time.duration_since(last_frame_time);
        last_frame_time = current_frame_time;

        // Convert delta_time to seconds as a floating-point number
        let delta_time = delta_time.as_secs() as f32 + delta_time.subsec_nanos() as f32 / 1_000_000_000.0;

        // events
        window.process_events(event, delta_time, control_flow);

        let projection: Matrix4<f32> = perspective(
            Deg(window.camera.zoom),
            WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
            0.1,
            100.0
        );

        let view: Matrix4<f32> = window.camera.get_view_matrix();

        let mut model: Matrix4<f32> = Matrix4::from_translation(test_cube_pos); //TODO
        let angle = 20.0;
        model = model * Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0).normalize(), Deg(angle));

        // render
        renderer.render(projection, view, model);


        window.context.swap_buffers().unwrap();
    });
}
