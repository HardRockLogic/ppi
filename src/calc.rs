#![allow(dead_code, unused_variables)]
use std::eprintln;

use super::parse::ScreenData;

struct ScreenEdges {
    width: f64,
    height: f64,
}

fn gcd(a: f64, b: f64) -> f64 {
    if b == 0. {
        a
    } else {
        gcd(b, a % b)
    }
}

impl ScreenEdges {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn diagonal_in_pixels(&self) -> f64 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }

    fn aspect_ratio(&self) -> (f64, f64) {
        let common = gcd(self.height, self.width);

        ((self.width / common), (self.height / common))
    }
}

pub struct PPIHandle {
    pub ppi: f64,
    pub ppi_square: f64,
    pub total_px: u32,
    pub aspect_ratio: (f64, f64),
}

impl PPIHandle {
    pub fn new() -> Self {
        let data: ScreenData = argh::from_env();

        let mut screen: Option<ScreenEdges> = None;

        let mut parsed_res_count: u8 = 0;

        if let Some(edges) = data.resolution {
            let width = edges[0] as f64;
            let height = edges[1] as f64;
            screen = Some(ScreenEdges::new(width, height));
            parsed_res_count += 1;
        }

        if data.hd {
            screen = Some(ScreenEdges::new(1280., 720.));
            parsed_res_count += 1;
        }
        if data.fhd {
            screen = Some(ScreenEdges::new(1920., 1080.));
            parsed_res_count += 1;
        }
        if data.qhd {
            screen = Some(ScreenEdges::new(2560., 1440.));
            parsed_res_count += 1;
        }
        if data.uhd {
            screen = Some(ScreenEdges::new(3840., 2160.));
            parsed_res_count += 1;
        }

        let total_px: u32;

        if parsed_res_count > 1 {
            eprintln!(
                "\nToo many resolution options were given, to list available options see --help note.\n"
            );
            std::process::exit(1);
        }

        let (diagonal_in_pixels, aspect_ratio) = match screen {
            Some(edges) => {
                total_px = edges.width as u32 * edges.height as u32;
                (edges.diagonal_in_pixels(), edges.aspect_ratio())
            }
            None => {
                eprintln!("\nNo resulution option were gieven, see --help note.\n");
                std::process::exit(1);
            }
        };

        let ppi = diagonal_in_pixels / data.diagonal as f64;
        let ppi_square = ppi * ppi;

        Self {
            ppi,
            ppi_square,
            total_px,
            aspect_ratio,
        }
    }
}
