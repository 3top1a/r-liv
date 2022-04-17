mod shaders;
mod ui;
mod utils;
mod window;

fn main() {
	println!("--- R-liv v{} ---", std::env!("CARGO_PKG_VERSION"));

	let args: Vec<String> = std::env::args().collect();

	window::main(args[args.len() - 1].clone());
}
