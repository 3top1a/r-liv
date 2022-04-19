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

pub fn get_file_name_from_path(filename: String) -> String {
	// TODO
	return filename;
}

/*Vertex {
							position: [0.0, 0.0],
							tex_coords: [0.0, 0.0],
						},
						Vertex {
							position: [0.0, 1.0],
							tex_coords: [0.0, 1.0],
						},
						Vertex {
							position: [1.0, 1.0],
							tex_coords: [1.0, 1.0],
						},
						Vertex {
							position: [1.0, 0.0],
							tex_coords: [1.0, 0.0],
						},
*/

pub const QUAD: [Vertex; 4] = [
	Vertex {
		position: [-1.0, -1.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		position: [-1.0, 1.0],
		uv: [0.0, 1.0],
	},
	Vertex {
		position: [1.0, 1.0],
		uv: [1.0, 1.0],
	},
	Vertex {
		position: [1.0, -1.0],
		uv: [1.0, 0.0],
	},
];
