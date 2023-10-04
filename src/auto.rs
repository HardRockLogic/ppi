pub(crate) mod linux {

    use nom::{
        bytes::complete::{tag, take_until, take_while},
        character::complete::{self as cc, char},
        sequence::{preceded, separated_pair},
        IResult,
    };
    use std::{
        eprintln,
        process::{self, Command},
    };

    fn find_dimensions(input: &[u8]) -> IResult<&[u8], (f32, f32)> {
        let (input, parsed) = take_until("mm")(input)?;
        let parsed_iter = parsed.split(|&b| b == b' ');
        let first_portion_utf8 = parsed_iter.last().unwrap();

        let (input, _) = take_while(|c| c != b'x')(input)?;
        let (input, _) = char('x')(input)?;

        let (leftover, parsed) = take_until("mm")(input)?;
        let parsed_iter = parsed.split(|&b| b == b' ');
        let second_portion_utf8 = parsed_iter.last().unwrap();

        let first_portion = std::str::from_utf8(first_portion_utf8)
            .expect("Internal. Failed to parse utf8.")
            .parse::<f32>()
            .expect("failed to parse number");
        let second_portion = std::str::from_utf8(second_portion_utf8)
            .expect("Internal. Failed to parse utf8.")
            .parse::<f32>()
            .expect("failed to parse number");

        Ok((leftover, (first_portion, second_portion)))
    }

    fn parse_res(i: &[u8]) -> IResult<&[u8], (u32, u32)> {
        let (leftover, (left, right)) = preceded(
            tag("current "),
            separated_pair(cc::u32, tag(" x "), cc::u32),
        )(i)?;

        Ok((leftover, (left, right)))
    }

    fn parse_current_res(i: &[u8]) -> IResult<&[u8], (u32, u32)> {
        let (remainder, _) = take_until("current")(i)?;

        parse_res(remainder)
    }

    // alternative more simple all in one parser.
    // require --listmonitors arg to be streamed into parser.
    // currently doesnt not solve issue with grabbing just first monitor
    // and not necessaril primary one.
    fn parse_all(i: &[u8]) -> IResult<&[u8], ((u32, u32), (u32, u32))> {
        let (rest, first_part) = take_until("x")(i)?;

        let left_portion = first_part.split(|&b| b == b' ').last().unwrap();

        let (_, (left_res, left_size)) = separated_pair(cc::u32, tag("/"), cc::u32)(left_portion)?;

        let (leftover, (right_res, right_size)) =
            preceded(tag("x"), separated_pair(cc::u32, tag("/"), cc::u32))(rest)?;

        Ok((leftover, ((left_res, right_res), (left_size, right_size))))
    }

    pub(crate) struct PseudoScreenData {
        pub(crate) diagonal: f32,
        pub(crate) resolution: [u32; 2],
        pub(crate) dims: [f32; 2],
    }

    impl PseudoScreenData {
        pub(crate) fn new() -> Self {
            let output = Command::new("xrandr").output().unwrap_or_else(|err| {
                eprintln!("Failed to envoke xrandr: {err:?}");
                process::exit(1);
            });

            let diagonal: f32;
            let resolution: [u32; 2];
            let dims: [f32; 2];

            // match parse_current_res(output.stdout.as_slice()) {
            //     Ok((_, (frst, sec))) => {
            //         resolution = [frst, sec];
            //     }
            //     Err(_) => {
            //         eprintln!("Pattern not found");
            //         process::exit(1);
            //     }
            // }
            //
            // match find_dimensions(output.stdout.as_slice()) {
            //     Ok((_, (frst, sec))) => {
            //         dims = [frst, sec];
            //         diagonal = (frst.powi(2) + sec.powi(2)).sqrt() * 0.0393700787;
            //     }
            //     Err(_) => {
            //         eprintln!("Pattern not found");
            //         process::exit(1);
            //     }
            // }

            match parse_all(output.stdout.as_slice()) {
                Ok((_, ((left_res, right_res), (left_size, right_size)))) => {
                    resolution = [left_res, right_res];
                    let left_dim = left_size as f32;
                    let right_dim = right_size as f32;

                    dims = [left_dim, right_dim];
                    diagonal = (left_dim.powi(2) + right_dim.powi(2)).sqrt() * 0.0393700787;
                }
                Err(_) => {
                    eprintln!("Pattern not found");
                    process::exit(1);
                }
            }

            Self {
                diagonal,
                resolution,
                dims,
            }
        }
    }
}
