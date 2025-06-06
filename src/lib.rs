use wasm_bindgen::prelude::*;

// Module declarations
pub mod renderer;
pub mod parser;

// Re-export main types for convenience
pub use renderer::Renderer;

// Entry point for the WASM module
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // Initialize the WebGPU renderer
    let renderer = Renderer::new().await?;
    
    // Render the initial frame
    renderer.render()?;
    
    Ok(())
}

// Export the init function for JavaScript
#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
    main().await
}
