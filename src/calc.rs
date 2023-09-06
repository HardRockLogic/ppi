use std::{eprintln, unreachable};

use super::args::ScreenData;
use crate::args::SubCommEnum;
use crate::auto::linux;

#[derive(Default)]
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

enum State {
    NotAssigned,
    Assigned,
    Redundant,
}

impl State {
    fn new() -> Self {
        Self::NotAssigned
    }

    fn update(self) -> Self {
        match self {
            Self::NotAssigned => Self::Assigned,
            Self::Assigned => Self::Redundant,
            Self::Redundant => self,
        }
    }
}

struct StateTracker(Option<State>);

impl StateTracker {
    fn new() -> Self {
        Self(Some(State::new()))
    }

    fn update(&mut self) {
        let state_ref = self.0.take();
        self.0 = Some(state_ref.unwrap().update());
    }
}

pub struct PPIHandle {
    pub ppi: f64,
    pub ppi_square: f64,
    pub total_px: u32,
    pub aspect_ratio: (f64, f64),
    pub diagonal_in_pixels: f64,
    pub dot_pitch: f64,
}

impl PPIHandle {
    pub fn new() -> Self {
        let data: ScreenData = argh::from_env();

        let mut screen = ScreenEdges::default();
        let mut state = StateTracker::new();

        let diagonal = match data.diagonal {
            Some(value) => value,
            None => match data.auto_subcommand {
                Some(SubCommEnum::SubCommAuto(auto)) => {
                    #[cfg(target_os = "linux")]
                    let pseudo_data = linux::PseudoScreenData::new();
                    let width = pseudo_data.resolution[0] as f64;
                    let height = pseudo_data.resolution[1] as f64;
                    screen = ScreenEdges::new(width, height);
                    state.update();

                    if auto.verbose {
                        let session_type = "XDG_SESSION_TYPE";
                        match std::env::var(session_type) {
                            Ok(val) => println!("Session type: {val}"),
                            Err(_) => eprintln!("{session_type} is not defined"),
                        }
                        println!("Found resolution: {} x {}", width as u32, height as u32);
                        println!(
                            "Found dimensions: {}mm x {}mm",
                            pseudo_data.dims[0] as u32, pseudo_data.dims[1] as u32
                        );
                        println!(
                            "Calculated diagonal: {}\"",
                            round::round(pseudo_data.diagonal as f64, 1)
                        );
                    }

                    pseudo_data.diagonal
                }
                None => {
                    eprintln!("\nNo diagonal option supplied\n");
                    std::process::exit(1);
                }
            },
        };

        if let Some(edges) = data.resolution {
            let width = edges[0] as f64;
            let height = edges[1] as f64;
            screen = ScreenEdges::new(width, height);
            state.update();
        }

        if data.hd {
            screen = ScreenEdges::new(1280., 720.);
            state.update();
        }
        if data.fhd {
            screen = ScreenEdges::new(1920., 1080.);
            state.update();
        }
        if data.qhd {
            screen = ScreenEdges::new(2560., 1440.);
            state.update();
        }
        if data.uhd {
            screen = ScreenEdges::new(3840., 2160.);
            state.update();
        }

        let total_px: u32;

        let (diagonal_in_pixels, aspect_ratio) = match state.0 {
            Some(State::NotAssigned) => {
                eprintln!("\nNo resulution option were gieven, to list available options run ppi --help.\n");
                std::process::exit(1);
            }
            Some(State::Assigned) => {
                total_px = screen.width as u32 * screen.height as u32;
                (screen.diagonal_in_pixels(), screen.aspect_ratio())
            }
            Some(State::Redundant) => {
                eprintln!(
                    "\nToo many resolution options were given, to list available options run ppi --help.\n"
                );
                std::process::exit(1);
            }
            None => unreachable!(),
        };

        let ppi = diagonal_in_pixels / diagonal as f64;
        let dot_pitch = (diagonal as f64 / diagonal_in_pixels) * 25.4;
        let ppi_square = ppi * ppi;

        Self {
            ppi,
            ppi_square,
            total_px,
            aspect_ratio,
            diagonal_in_pixels,
            dot_pitch,
        }
    }
}
