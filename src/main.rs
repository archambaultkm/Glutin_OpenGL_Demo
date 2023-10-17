mod shader;

extern crate gl;
use gl::types::*;

extern crate glutin;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;

use std::ffi::CString;
use std::mem;

use glutin_opengl_demo::{polygon_mode, PolygonMode};
use shader::Shader;

// settings
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;

fn main() {
    // Initialize the event loop and window builder
    //the event loop handles events such as keyboard and mouse input, window resizing, and more.
    let event_loop = EventLoop::new();

    //configure new window properties with windowbuilder
    let window = WindowBuilder::new()
        .with_title("OpenGL and Glutin Demo")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

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
        shader_program = Shader::new("shaders/shader.vs", "shaders/shader.fs");
        gl::UseProgram(shader_program.ID);

        //define vertices of two triangles to draw a rectangle
        //there will be overlap between the two triangles,
        let vertices: [f32; 24] = [
            // first triangle
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, // top right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 0.0, 0.0, 1.0 // top left
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
        define_buffer(
            gl::ARRAY_BUFFER,
            &vertices,
            gl::STATIC_DRAW
        );

        // Generate and bind element buffer object (EBO)
        define_buffer(
            gl::ELEMENT_ARRAY_BUFFER,
            &indices,
            gl::STATIC_DRAW
        );

        let stride = (6 * mem::size_of::<f32>()) as GLsizei;
        define_attrib_pointers(shader_program.ID, stride);
    }

    // turn on polygon mode:
    polygon_mode(PolygonMode::Line);

    // Main event loop runs until application is terminated.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // events
        process_events(event, control_flow);

        // render
        unsafe {
            // window background colour
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

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

    let colour_attr_location = gl::GetAttribLocation(
        shader_program_id,
        CString::new("colour").unwrap().as_ptr()
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

    // colour attribute
    gl::VertexAttribPointer(
        colour_attr_location as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
    );
    gl::EnableVertexAttribArray(colour_attr_location as GLuint);
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
