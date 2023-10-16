extern crate gl;
extern crate glutin;

use std::ffi::CString;
use gl::types::*;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin_opengl_demo::{polygon_mode, PolygonMode};

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

    unsafe {
        // window background colour
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);

        // Create vertex and fragment shaders written in GLSL
        //vertex shader processes each vertex's position
        let vertex_shader_source = r#"
            #version 330 core
            layout(location = 0) in vec3 position;
            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;

        //fragment shader sets the colour of each fragment (pixel)
        let fragment_shader_source = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        // Compile and link shaders
        let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
        let shader_program = create_shader_program(vertex_shader, fragment_shader);

        gl::UseProgram(shader_program);

        //define vertices of two triangles to draw a rectangle
        //there will be overlap between the two triangles,
        let vertices: [f32; 12] = [
            // first triangle
            0.5,  0.5, 0.0,  // top right
            0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5,  0.5, 0.0,  // top left
        ];

        //so it's more efficient to only include each vertice once and then use
        //an EBO to specify draw order.
        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        //generate and bind vertex array object (VAO)
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        //Generate and bind vertex buffer object (VBO)
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

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
        let pos_attr_location = gl::GetAttribLocation(shader_program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr_location as GLuint);
        gl::VertexAttribPointer(
            pos_attr_location as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null(),
        );
    }

    // turn on polygon mode:
     polygon_mode(PolygonMode::Line);

    // Main event loop runs until application is terminated.
    event_loop.run(move |event, _, control_flow| {
        //equivalent to control_flow.set_wait() on newer versions
        *control_flow = ControlFlow::Poll;

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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
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

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        // Create a new shader object
        let shader = gl::CreateShader(shader_type);

        // Set the shader source and compile it
        gl::ShaderSource(shader, 1, &CString::new(source).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            // Compilation failed, get the error message
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len(log_length as usize - 1); // Subtract 1 to ignore the null terminator

            gl::GetShaderInfoLog(shader, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let error_message = String::from_utf8_lossy(&log);
            println!("Shader compilation error: {}", error_message);
        }

        shader
    }
}

fn create_shader_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        // Create a new shader program
        let shader_program = gl::CreateProgram();

        // Attach the vertex and fragment shaders to the program
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);

        // Link the shader program
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            // Linking failed, get the error message
            let mut log_length = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len(log_length as usize - 1); // Subtract 1 to ignore the null terminator

            gl::GetProgramInfoLog(shader_program, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let error_message = String::from_utf8_lossy(&log);
            println!("Shader program linking error: {}", error_message);
        }

        // Detach and delete the individual shaders since they are now part of the program
        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}