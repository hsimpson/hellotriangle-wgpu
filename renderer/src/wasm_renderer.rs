use crate::renderer::{Renderer, Window};

use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WASMRenderer {
    renderer: Renderer,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl WASMRenderer {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub async fn new(canvas: HtmlCanvasElement) -> Self {
        let window = Window::from_canvas(&canvas);

        let renderer = Renderer::new(window, canvas.width(), canvas.height()).await;

        Self { renderer }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn update(&mut self) {
        self.renderer.update();
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn render(&mut self) {
        match self.renderer.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
            // The system is out of memory, we should probably quit
            // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => log::error!("{:?}", e),
        }
    }
}
