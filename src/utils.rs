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
