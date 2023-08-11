#![allow(dead_code)]

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Screen data
pub struct ScreenData {
    /// screen diagonal value
    #[argh(positional)]
    diagonal: u32,
    /// custom resolution
    #[argh(option, short = 'r', from_str_fn(parse_resolution))]
    resulution: Option<[u32; 2]>,
    /// standart 16:9 hd resollution
    #[argh(switch, short = 'h')]
    hd: bool,
    /// standart 16:9 full hd resollution (1920x1080)
    #[argh(switch, short = 'f')]
    fhd: bool,
    /// standart 16:9 quad hd resollution
    #[argh(switch, short = 'q')]
    qhd: bool,
    /// standart 16:9 ultra hd resollution
    #[argh(switch, short = 'u')]
    uhd: bool,
}

fn parse_resolution(i: &str) -> Result<[u32; 2], String> {
    let mut iter = i.split("x");
    let a = iter.next().unwrap().parse::<u32>().unwrap();
    let b = iter.next().unwrap().parse::<u32>().unwrap();

    Ok([a, b])
}
