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

// Quite a bit of this code has been copied from https://github.com/hakolao/egui_winit_vulkano/blob/master/examples/minimal.rs
// Which is copyrighted under MIT

use std::sync::Arc;
use vulkano::{
	device::{
		physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Features, Queue,
		QueueCreateInfo,
	},
	image::{view::ImageView, ImageUsage, SwapchainImage},
	swapchain::{PresentMode, Surface, Swapchain, SwapchainCreateInfo},
};
use winit::window::Window;

pub fn create_device(
	physical: PhysicalDevice,
	surface: Arc<vulkano::swapchain::Surface<Window>>,
) -> (Arc<Device>, Arc<Queue>) {
	let queue_family = physical
		.queue_families()
		.find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
		.expect("couldn't find a graphical queue family");
	// Add device extensions based on needs
	let device_extensions = DeviceExtensions {
		khr_swapchain: true,
		..DeviceExtensions::none()
	};
	// Add device features
	let features = Features::none();
	let (device, mut queues) = {
		Device::new(
			physical,
			DeviceCreateInfo {
				enabled_extensions: physical.required_extensions().union(&device_extensions),
				enabled_features: features,
				queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
				_ne: Default::default(),
			},
		)
		.expect("failed to create device")
	};
	(device, queues.next().unwrap())
}

pub fn create_swap_chain(
	surface: Arc<Surface<Window>>,
	physical: PhysicalDevice,
	device: Arc<Device>,
) -> (
	Arc<Swapchain<Window>>,
	Vec<Arc<ImageView<SwapchainImage<Window>>>>,
) {
	let surface_capabilities = physical
		.surface_capabilities(&surface, Default::default())
		.unwrap();
	let image_format = Some(
		physical
			.surface_formats(&surface, Default::default())
			.unwrap()[0]
			.0,
	);
	let image_extent = surface.window().inner_size().into();

	let (swapchain, images) = Swapchain::new(
		device,
		surface,
		SwapchainCreateInfo {
			min_image_count: surface_capabilities.min_image_count,
			image_format,
			image_extent,
			image_usage: ImageUsage::color_attachment(),
			composite_alpha: surface_capabilities
				.supported_composite_alpha
				.iter()
				.next()
				.unwrap(),
			..Default::default()
		},
	)
	.unwrap();
	let images = images
		.into_iter()
		.map(|image| ImageView::new_default(image).unwrap())
		.collect::<Vec<_>>();
	(swapchain, images)
}
