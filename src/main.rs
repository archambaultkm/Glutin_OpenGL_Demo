mod shader;

extern crate gl;
extern crate glutin;

use std::ffi::CString;
use std::mem;
use gl::types::*;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin_opengl_demo::{polygon_mode, PolygonMode};

use shader::Shader;

fn main() {
    // Initialize the event loop and window builder
    //the event loop handles events such as keyboard and mouse input, window resizing, and more.
    let event_loop = EventLoop::new();

    //configure new window properties with windowbuilder
    let window = WindowBuilder::new()
        .with_title("OpenGL and Glutin Demo")
        .with_inner_size(LogicalSize::new(800, 600));

    //create opengl context within the glutin window and set as current context.
    let context = unsafe {
        ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap()
            .make_current()
    }
        //unwrap is a cheap way to handle errors
        .unwrap();

    // Initialize OpenGL (make opengl functions available within the program)
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let mut shader_program: Shader;
    let mut vao : GLuint = 0;
    unsafe {
        // window background colour
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);

        shader_program = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        gl::UseProgram(shader_program.ID);

        //define vertices of two triangles to draw a rectangle
        //there will be overlap between the two triangles,
        let vertices: [f32; 24] = [
            // first triangle
            0.5,  0.5, 0.0,  1.0, 0.0, 0.0,// top right
            0.5, -0.5, 0.0,  0.0, 1.0, 0.0,// bottom right
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,// bottom left
            -0.5,  0.5, 0.0, 0.0, 0.0, 1.0// top left
        ];

        //so it's more efficient to only include each vertice once and then use
        //an EBO to specify draw order.
        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        // Generate and bind vertex array object (VAO)
        //let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generate and bind vertex buffer object (VBO)
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        // Generate and bind element buffer object (EBO)
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        // Define the vertex attribute pointer for the position attribute
        let pos_attr_location = gl::GetAttribLocation(shader_program.ID, CString::new("position").unwrap().as_ptr());
        //gl::EnableVertexAttribArray(pos_attr_location as GLuint);

        let stride = (6 * mem::size_of::<f32>()) as GLsizei;
        // position attribute
        gl::VertexAttribPointer(
            pos_attr_location as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // colour attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    // turn on polygon mode:
    polygon_mode(PolygonMode::Fill);

    // Get the current time
    let start_time = std::time::Instant::now();
    let offset : f32 = 0.5;

    // Main event loop runs until application is terminated.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // events
        process_events(event, control_flow);

        // render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.set_float(&CString::new("x_offset").unwrap(), offset);
            // // update shader uniform
            // // Get the elapsed time as a Duration
            // let elapsed_time = start_time.elapsed();
            // // Convert the elapsed time to an f32 (in seconds)
            // let time_value = elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 / 1_000_000_000.0;
            //
            // let green_value = time_value.sin() / 2.0 + 0.5 + time_value;
            // let my_colour = CString::new("my_colour").unwrap();
            // let vertex_color_location = gl::GetUniformLocation(shader_program.ID, my_colour.as_ptr());
            // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const _
            );
        }

        context.swap_buffers().unwrap();
    });
}

fn process_events(event : Event<()>, control_flow : &mut ControlFlow) {
// events
    match event {
        //match statement waits until the user presses exit and reacts to that event by setting the
        //control flow to exit (close the window)
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,

        //This is a catch-all case in the match statement like finally in switch
        _ => (),
    }
}
