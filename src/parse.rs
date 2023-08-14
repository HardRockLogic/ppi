#![allow(dead_code)]

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Screen data
pub struct ScreenData {
    /// screen diagonal value
    #[argh(positional)]
    pub(crate) diagonal: f32,
    /// custom resolution in format 1366x768 where x is any alphabetic character
    #[argh(option, short = 'r', from_str_fn(parse_resolution))]
    pub(crate) resolution: Option<[u32; 2]>,
    /// standart 16:9 hd resollution
    #[argh(switch, short = 'h')]
    pub(crate) hd: bool,
    /// standart 16:9 full hd resollution (1920x1080)
    #[argh(switch, short = 'f')]
    pub(crate) fhd: bool,
    /// standart 16:9 quad hd resollution
    #[argh(switch, short = 'q')]
    pub(crate) qhd: bool,
    /// standart 16:9 ultra hd resollution
    #[argh(switch, short = 'u')]
    pub(crate) uhd: bool,
}

fn parse_resolution(i: &str) -> Result<[u32; 2], String> {
    let chr = i
        .chars()
        .find(|ch| ch.is_alphabetic())
        .ok_or(Err("Expected alphabetic delimetr".to_string()));

    let chr = match chr {
        Ok(delimetr) => delimetr,
        Err(err) => return err,
    };

    let mut iter = i.split(chr);
    let a = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .map_err(|_| Err("Resolution should be numeric".to_string()));
    let b = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .map_err(|_| Err("Resolution should be numeric".to_string()));

    let a = match a {
        Ok(value) => value,
        Err(err) => {
            return err;
        }
    };
    let b = match b {
        Ok(value) => value,
        Err(err) => {
            return err;
        }
    };

    Ok([a, b])
}
