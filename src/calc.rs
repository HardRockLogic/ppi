#![allow(dead_code, unused_variables)]
use std::eprintln;

use super::parse::ScreenData;

struct ScreenEdges {
    width: f32,
    height: f32,
}

impl ScreenEdges {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    fn diagonal_in_pixels(&self) -> f32 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

pub struct PPIHandle {
    pub ppi: f32,
    pub ppi_square: f32,
    pub total_px: u32,
}

impl PPIHandle {
    pub fn new() -> Self {
        let data: ScreenData = argh::from_env();

        let mut screen: Option<ScreenEdges> = None;

        if let Some(edges) = data.resolution {
            let width = edges[0] as f32;
            let height = edges[1] as f32;
            screen = Some(ScreenEdges::new(width, height));
        }

        if data.fhd {
            screen = Some(ScreenEdges::new(1920., 1080.));
        }
        if data.qhd {
            screen = Some(ScreenEdges::new(2560., 1440.));
        }
        if data.uhd {
            screen = Some(ScreenEdges::new(3840., 2160.));
        }

        let total_px: u32;

        let diagonal_in_pixels = match screen {
            Some(edges) => {
                total_px = edges.width as u32 * edges.height as u32;
                edges.diagonal_in_pixels()
            }
            None => {
                eprintln!("\nNo resulution option were gieven, see --help note.\n");
                std::process::exit(1);
            }
        };

        let ppi = diagonal_in_pixels / data.diagonal;
        let ppi_square = ppi * ppi;

        Self {
            ppi,
            ppi_square,
            total_px,
        }
    }
}
