#![warn(warnings)]

use clap::Parser;

#[derive(Parser)]
struct Opt {
    device: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    let ws2300 = ws2300::Device::new(opt.device)?;

    let data = ws2300.read_all()?;

    let json = serde_json::to_string(&data)?;
    println!("{json}");

    Ok(())
}
