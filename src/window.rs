use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::camera::{Camera, Camera_Movement::*};

pub fn process_events(
    event : Event<()>,
    camera: &mut Camera,
    delta_time : f32,
    control_flow : &mut ControlFlow) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }

            WindowEvent::KeyboardInput { input, .. } => {
                process_input(input, delta_time, camera);
            }

            _ => {}
        }

        //This is a catch-all case in the match statement like finally in switch
        _ => (),
    }
}

pub fn process_input(input : KeyboardInput, delta_time : f32, camera : &mut Camera) {
    if let Some(key_code) = input.virtual_keycode {
        match key_code {
            VirtualKeyCode::Escape => {
                if input.state == ElementState::Pressed {
                    // Set the window to close when Escape key is pressed.
                    // Note: You'll need to handle window closing separately in your event loop.
                    // For example, you can set the control flow to ControlFlow::Exit.
                    // control_flow = ControlFlow::Exit;
                }
            }
            VirtualKeyCode::W => {
                if input.state == ElementState::Pressed {
                    camera.process_keyboard(FORWARD, delta_time);
                }
            }
            VirtualKeyCode::S => {
                if input.state == ElementState::Pressed {
                    camera.process_keyboard(BACKWARD, delta_time);
                }
            }
            VirtualKeyCode::A => {
                if input.state == ElementState::Pressed {
                    camera.process_keyboard(LEFT, delta_time);
                }
            }
            VirtualKeyCode::D => {
                if input.state == ElementState::Pressed {
                    camera.process_keyboard(RIGHT, delta_time);
                }
            }
            _ => {}
        }
    }
}