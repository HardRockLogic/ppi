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

    fn ppi(&self) -> f32 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

pub struct PPIHandle {
    pub ppi: f32,
}

impl PPIHandle {
    pub fn ppi_calc() -> Self {
        let data: ScreenData = argh::from_env();

        let mut screen: Option<ScreenEdges> = None;

        if let Some(edges) = data.resolution {
            let width = edges[0] as f32;
            let height = edges[1] as f32;
            screen = Some(ScreenEdges::new(width, height));
        }

        let diagonal_in_pixels = match screen {
            Some(edges) => edges.ppi(),
            None => {
                eprintln!("\nNo resulution option were gieven, see --help note.\n");
                std::process::exit(1);
            }
        };

        let ppi = diagonal_in_pixels / data.diagonal as f32;

        Self { ppi }
    }
}
