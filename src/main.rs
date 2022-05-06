mod shaders;
mod renderer;
mod utils;
mod window;
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("--- R-liv v{} ---", std::env!("CARGO_PKG_VERSION"));

	let args: Vec<String> = std::env::args().collect();

	window::main(args[args.len() - 1].clone());

    Ok(())
}
