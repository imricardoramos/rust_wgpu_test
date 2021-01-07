use winit::{
    event_loop::ControlFlow,
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
};

use crate::window;
use crate::scene;

pub fn run(){
    let (event_loop, window) = window::create_window();
    let mut scene = scene::Scene::new(&window);
    
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                ref event, 
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested  => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        scene.redraw(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        scene.redraw(**new_inner_size);
                    },
                    _ => {},
                }
            },
            Event::RedrawRequested(_) => {
                match scene.render(&window) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    // Err(wgpu::SwapChainError::Lost) => scene.redraw(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
        scene.gui.platform.handle_event(scene.gui.imgui.io_mut(), &window, &event);
    });
}
