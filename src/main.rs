mod shader;
mod camera;
mod game_window;
mod cube;
mod texture;
mod game_specs;
mod world;

extern crate gl;
use gl::types::*;

extern crate glutin;
use glutin::event_loop::{ControlFlow, EventLoop};

extern crate cgmath;
use cgmath::{Deg, InnerSpace, Matrix, Matrix4, perspective, Point3, SquareMatrix, vec3, Vector3};

use std::ffi::CString;
use std::mem;

use glutin_opengl_demo::{polygon_mode, PolygonMode};

use shader::Shader;
use cube::Cube;
use crate::texture::Texture;
use game_window::GameWindow;
use crate::game_specs::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::world::World;


fn main() {

    // Initialize the event loop and window builder
    //the event loop handles events such as keyboard and mouse input, window resizing, and more.
    let event_loop = EventLoop::new();

    let mut window = GameWindow::new();

    // Initialize OpenGL (make opengl functions available within the program)
    gl::load_with(|symbol| window.context.get_proc_address(symbol) as *const _);

    let world = World::new();
    let test_cube_pos = Vector3::new(0.0, 0.0, 0.0);

    //TODO this should get moved
    let mut shader_program: Shader;
    let mut vao : GLuint = 0;
    let mut texture1 : Texture;

    unsafe {
        shader_program = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        gl::UseProgram(shader_program.ID);
        gl::Enable(gl::DEPTH_TEST);

        // Generate and bind vertex array object (VAO)
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        for cube in world.objects {
            // Generate and bind vertex buffer object (VBO)
            define_buffer(
                gl::ARRAY_BUFFER,
                &cube.vertices,
                gl::STATIC_DRAW
            );
        }

        //this should get moved to shader struct probably?
        // define attribute pointers
        //TODO hard-coding for now
        let stride = (5 * mem::size_of::<GLfloat>()) as GLsizei;
        define_attrib_pointers(shader_program.ID, stride);

        texture1 = Texture::new("resources/textures/wall.jpeg");

        //assign shader sampler to texture unit
        shader_program.set_int(&CString::new("texture1").unwrap(), 0);
    }

    // Initialize variables for tracking time
    let mut last_frame_time = std::time::Instant::now();
    let mut delta_time = std::time::Duration::new(0, 0);

    // "settings"
    unsafe { gl::ClearColor(0.7, 0.7, 0.8, 1.0); }
    polygon_mode(PolygonMode::Fill);

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

        // render
        unsafe {

            // window background colour
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1.id);

            let projection: Matrix4<f32> = perspective(
                Deg(window.camera.zoom),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0
            );

            let view: Matrix4<f32> = window.camera.get_view_matrix();

            // pass to the shaders
            shader_program.set_mat4(&CString::new("view").unwrap(), &view);
            shader_program.set_mat4(&CString::new("projection").unwrap(), &projection);

            // draw
            gl::BindVertexArray(vao);

            let mut model: Matrix4<f32> = Matrix4::from_translation(test_cube_pos); //TODO
            let angle = 20.0;
            model = model * Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0).normalize(), Deg(angle));
            shader_program.set_mat4(&CString::new("model").unwrap(), &model);

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                36
            );
        }

        window.context.swap_buffers().unwrap();
    });
}

//generate and bind buffer objects for both VBO and EBO
fn define_buffer<T>(target: GLenum, array : &[T], draw_type : GLenum) -> GLuint {
    let mut buffer_object = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer_object);
        gl::BindBuffer(target, buffer_object);
        gl::BufferData(
            target,
            (array.len() * std::mem::size_of::<T>()) as isize,
            array.as_ptr() as *const std::ffi::c_void,
            draw_type,
        );
    }

    buffer_object
}

unsafe fn define_attrib_pointers(shader_program_id : GLuint, stride : GLsizei) {
    let pos_attr_location = gl::GetAttribLocation(
        shader_program_id,
        CString::new("position").unwrap().as_ptr()
    );

    let texture_attr_location = gl::GetAttribLocation(
        shader_program_id,
        CString::new("texture").unwrap().as_ptr()
    );

    // position attribute
    gl::VertexAttribPointer(
        pos_attr_location as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        std::ptr::null(),
    );
    gl::EnableVertexAttribArray(pos_attr_location as GLuint);

    // texture attribute
    gl::VertexAttribPointer(
        texture_attr_location as GLuint,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
    );
    gl::EnableVertexAttribArray(texture_attr_location as GLuint);
}
