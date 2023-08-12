use std::println;

use ppi::PPIHandle;

fn main() {
    let handle = PPIHandle::ppi_calc();

    println!("\nscreen PPI is: {}\n", handle.ppi);
}
