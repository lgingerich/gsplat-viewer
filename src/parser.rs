use anyhow::Result;
use std::{fs::File, io::{BufRead, BufReader, Read}, path::Path, mem::size_of};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawSplat {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub f_dc: [f32; 3],
    pub f_rest: [f32; 45],
    pub opacity: f32,
    pub log_scale: [f32; 3],
    pub rot: [f32; 4],
}

// NOTE: Can only parse .ply files with the format "binary"
pub fn load_ply(path: &Path) -> Result<Vec<RawSplat>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut format = String::new();
    let mut vertex_count = 0usize;

    // Read header
    loop {
        line.clear();
        reader.read_line(&mut line)?;

        if line.starts_with("format") {
            format = line.split_whitespace().nth(1).unwrap().to_string();
        }
        if line.starts_with("element vertex") {
            vertex_count = line.split_whitespace().nth(2).unwrap().parse::<usize>()?;
        }
        if line.trim() == "end_header" { break }
    }

    let format_type = if format.starts_with("binary") {
        "binary"
    } else if format.starts_with("ascii") {
        "ascii"
    } else {
        return Err(anyhow::anyhow!("Invalid format: {}", format));
    };

    match format_type {
        "binary" => {
            // Read vertices
            let mut bytes = vec![0u8; vertex_count * size_of::<RawSplat>()];
            reader.read_exact(&mut bytes)?;

            let mut splats: Vec<RawSplat> = bytemuck::cast_slice(&bytes).to_vec();

            for s in &mut splats {
                // exponentiate scale, sigmoid opacity
                s.log_scale = s.log_scale.map(|l| l.exp());
                s.opacity    = 1.0 / (1.0 + (-s.opacity).exp());
            }

            Ok(splats)
        }
        "ascii" => {
            !unimplemented!()
        }
        _ => {
            !unreachable!()
        }
    }
}