use std::dbg;

use ppi::ScreenData;

fn main() {
    let ppi: ScreenData = argh::from_env();

    dbg!(ppi);
}
