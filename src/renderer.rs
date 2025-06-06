use crate::parser::RawSplat;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    splats: Vec<RawSplat>,
    camera_position: [f32; 3],
    camera_rotation: [f32; 4], // quaternion
    camera_intrinsics: [f32; 9], // intrinsic matrix
}

#[wasm_bindgen]
impl Renderer {
    pub async fn new() -> Result<Renderer, JsValue> {
        console_error_panic_hook::set_once();
        console_log!("Initializing Gaussian Splat Renderer...");

        // Get the window and document
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // Get the canvas element
        let canvas = document
            .get_element_by_id("webgpu-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        // Check if WebGPU is supported
        let navigator = window.navigator();
        let gpu = js_sys::Reflect::get(&navigator, &"gpu".into())?;
        
        if gpu.is_undefined() {
            console_log!("WebGPU is not supported in this browser");
            console_log!("This demo will show a placeholder instead");
        } else {
            console_log!("WebGPU is supported!");
        }

        Ok(Renderer {
            canvas,
            splats: vec![],
            camera_position: [0.0, 0.0, 0.0],
            camera_rotation: [0.0, 0.0, 0.0, 1.0],
            camera_intrinsics: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        })
    }

    pub fn render(&self) -> Result<(), JsValue> {
        console_log!("Rendering...");
        // Set up a simple 2D context for fallback
        let context = self.canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        // Clear canvas
        context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);

        // Set background
        context.set_fill_style_str("#1a1a1a");
        context.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        
        Ok(())
    }

    fn update_render(&self) {
        if let Err(e) = self.render() {
            console_log!("Render error: {:?}", e);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            console_log!("Canvas resized to {}x{}", width, height);
            self.update_render();
        }
    }
}

// Internal implementation not exposed to WASM
impl Renderer {
    pub fn load_splats(&mut self, splats: Vec<RawSplat>) {
        console_log!("Loading {} splats", splats.len());
        self.splats = splats;
        self.update_render();
    }
}
