use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use renderer::Renderer;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Native Rust")
        .build(&event_loop)
        .unwrap();

    let width = window.inner_size().width;
    let height = window.inner_size().height;

    let mut renderer = Renderer::new(&window, width, height).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                renderer.resize((*physical_size).width, (*physical_size).height);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize((**new_inner_size).width, (**new_inner_size).height);
            }
            _ => (),
        },
        Event::RedrawRequested(_) => {
            // renderer.update();
            match renderer.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => log::error!("{:?}", e),
            }
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        _ => (),
    });
}

fn main() {
    pollster::block_on(run());
}
