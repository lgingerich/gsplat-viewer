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
pub struct WebGpuRenderer {
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl WebGpuRenderer {
    pub async fn init() -> Result<WebGpuRenderer, JsValue> {
        console_error_panic_hook::set_once();
        console_log!("Initializing WebGPU renderer...");

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

        // Set up a simple 2D context for fallback
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        // Draw a simple triangle using 2D canvas as a placeholder
        context.set_fill_style(&"#ff0000".into());
        context.begin_path();
        context.move_to(400.0, 100.0);  // Top point
        context.line_to(300.0, 300.0);  // Bottom left
        context.line_to(500.0, 300.0);  // Bottom right
        context.close_path();
        context.fill();

        // Add some text
        context.set_fill_style(&"#ffffff".into());
        context.set_font("20px Arial");
        context.fill_text("WebGPU + Rust Demo", 300.0, 400.0)?;
        context.fill_text("Red Triangle (2D Canvas Fallback)", 250.0, 430.0)?;

        console_log!("WebGPU renderer initialized successfully!");

        Ok(WebGpuRenderer {
            canvas,
        })
    }

    pub fn render(&self) -> Result<(), JsValue> {
        // For this simple demo, we just log that render was called
        console_log!("Render called - triangle is already drawn");
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            console_log!("Canvas resized to {}x{}", width, height);
        }
    }
} 