pub mod parser;

use crate::parser::{load_ply, RawSplat};
use anyhow::Result;
use std::path::Path;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

fn main() {
    let path = Path::new("assets/train.ply");
    let gsp = load_ply(&path).unwrap();
    println!("Loaded {} splats", gsp.len());
    println!("{:?}", gsp[0]);

    let vertices = make_point_vertices(&gsp);
    println!("Made {} vertices", vertices.len());
    println!("{:?}", vertices[0]);

}

fn make_point_vertices(splats: &[RawSplat]) -> Vec<Vertex> {
    splats
        .iter()
        .map(|s| Vertex {
            pos: s.pos,
            color: s.f_dc,
        })
        .collect()
}
