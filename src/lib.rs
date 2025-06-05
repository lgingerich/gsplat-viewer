use wasm_bindgen::prelude::*;

// Module declarations
pub mod webgpu;
pub mod parser;

// Re-export main types for convenience
pub use webgpu::WebGpuRenderer;

// Entry point for the WASM module
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // Initialize the WebGPU renderer
    let renderer = WebGpuRenderer::init().await?;
    
    // Render the initial frame
    renderer.render()?;
    
    Ok(())
}

// Export the init function for JavaScript
#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
    main().await
}
