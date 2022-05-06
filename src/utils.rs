#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	pub position: [f32; 2],
	pub uv: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, uv);

extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use std::path::Path;

pub fn get_file_stem<P: AsRef<Path>>(path: P) -> String {
	path.as_ref()
		.file_stem()
		.map(|ext| ext.to_os_string())
		.unwrap_or_else(|| "".into())
		.into_string()
		.unwrap()
}

pub const QUAD: [Vertex; 3] = [
	Vertex {
		position: [-1.0, -1.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		position: [-0.5, 1.0],
		uv: [1.0, 1.0],
	},
	Vertex {
		position: [-1.0, 0.5],
		uv: [0.0, 1.0],
	},
];
