use cli_table::{format::Justify, Cell, Style, Table};
use ppi::PPIHandle;

fn main() {
    let handle = PPIHandle::new();
    let ppi_rounded = format!("{:.2}", handle.ppi);
    let ppi_square_rounded = format!("{:.2}", handle.ppi_square);
    let total_px_pretty = format_with_commas(handle.total_px);

    let table = vec![
        vec!["PPI".cell(), ppi_rounded.cell().justify(Justify::Right)],
        vec![
            "PPI²".cell(),
            ppi_square_rounded.cell().justify(Justify::Right),
        ],
        vec![
            "Total Px".cell(),
            total_px_pretty.cell().justify(Justify::Right),
        ],
    ]
    .table()
    .title(vec!["Propery".cell().bold(true), "Value".cell().bold(true)])
    .bold(true);

    let display = table.display().unwrap();

    println!("{display}");
}

fn format_with_commas<T: ToString>(input: T) -> String {
    let binding_chunk = input.to_string().chars().rev().collect::<Vec<_>>();
    let chunked_iter = binding_chunk.chunks(3);

    let chunks: Vec<String> = chunked_iter
        .rev()
        .map(|chunk| chunk.iter().rev().collect())
        .collect();

    chunks.join(",")
}
