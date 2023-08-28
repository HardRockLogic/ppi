use cli_table::{format::Justify, Cell, Color, Style, Table};
use ppi::PPIHandle;
use round::round;

fn main() {
    let handle = PPIHandle::new();
    let ppi_pretty = format!("{:.2}", handle.ppi);
    let ppi_square_pretty = format_with_commas(round(handle.ppi_square, 2));
    let total_px_pretty = format_with_commas(handle.total_px);
    let aspect_ratio_pretty = format!(
        "{}/{} ({:.2}:1)",
        handle.aspect_ratio.0 as u32,
        handle.aspect_ratio.1 as u32,
        handle.aspect_ratio.0 / handle.aspect_ratio.1
    );
    let diagonal_pix_pretty = format_with_commas(handle.diagonal_in_pixels.round());
    let dot_pitch_pretty = round(handle.dot_pitch, 4);

    let table = vec![
        vec![
            "PPI".cell(),
            ppi_pretty
                .cell()
                .justify(Justify::Right)
                .foreground_color(Some(Color::Red)),
        ],
        vec![
            "PPI²".cell(),
            ppi_square_pretty.cell().justify(Justify::Right),
        ],
        vec![
            "Total Px".cell(),
            total_px_pretty.cell().justify(Justify::Right),
        ],
        vec![
            "Aspect ratio".cell(),
            aspect_ratio_pretty.cell().justify(Justify::Right),
        ],
        vec![
            "Diagonal Px".cell(),
            diagonal_pix_pretty.cell().justify(Justify::Right),
        ],
        vec![
            "Dot pitch".cell(),
            dot_pitch_pretty.cell().justify(Justify::Right),
        ],
    ]
    .table()
    .title(vec![
        "Property".cell().bold(true),
        "Value".cell().bold(true),
    ])
    .bold(true);

    let display = table.display().unwrap();

    println!("{display}");
}

fn format_with_commas<T: ToString>(input: T) -> String {
    let stringed = input.to_string();

    let mut after_period: Option<_> = None;
    let mut base: Option<_> = None;

    if stringed.contains(".") {
        let mut iter = stringed.split(".");
        base = Some(iter.next().unwrap());
        after_period = Some(iter.next().unwrap());
    }

    let binding_chunk: Vec<_> = match base {
        Some(num) => num.chars().rev().collect::<Vec<_>>(),
        None => stringed.chars().rev().collect::<Vec<_>>(),
    };

    let chunked_iter = binding_chunk.chunks(3);

    let chunks: Vec<String> = chunked_iter
        .rev()
        .map(|chunk| chunk.iter().rev().collect())
        .collect();

    let mut output = chunks.join(",");
    if let Some(remainder) = after_period {
        output.push('.');
        output.push_str(remainder);
    }

    output
}
