extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageAccess, SwapchainImage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{
    self, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;

use vulkano_win::VkSurfaceBuild;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	pub position: [f32; 2],
	pub uv: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, uv);

pub struct WindowData {
	// Surface to draw onto
	surface: std::sync::Arc<vulkano::swapchain::Surface<winit::window::Window>>,
}

impl WindowData {
	pub fn new() -> (WindowData, winit::event_loop::EventLoop<()>) {
		let required_extensions = vulkano_win::required_extensions();

		let instance = vulkano::instance::Instance::new(vulkano::instance::InstanceCreateInfo {
			enabled_extensions: required_extensions,
			..Default::default()
		})
		.unwrap();

		let device_extensions = vulkano::device::DeviceExtensions {
			khr_swapchain: true,
			..vulkano::device::DeviceExtensions::none()
		};

		let event_loop = winit::event_loop::EventLoop::new();

		let surface = winit::window::WindowBuilder::new()
			.build_vk_surface(&event_loop, instance.clone())
   			.unwrap();

		let (physical_device, queue_family) = vulkano::device::physical::PhysicalDevice::enumerate(&instance)
			.filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
			.filter_map(|p| {
				p.queue_families()
					.find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
					.map(|q| (p, q))
			})
			.min_by_key(|(p, _)| match p.properties().device_type {
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu => 0,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu => 1,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu => 2,
				vulkano::device::physical::PhysicalDeviceType::Cpu => 3,
				vulkano::device::physical::PhysicalDeviceType::Other => 4,
			})
			.expect("no device available");

		let (device, mut queues) = vulkano::device::Device::new(
			physical_device,
			vulkano::device::DeviceCreateInfo {
				enabled_extensions: physical_device
					.required_extensions()
					.union(&device_extensions),

				queue_create_infos: vec![vulkano::device::QueueCreateInfo::family(queue_family)],
				..Default::default()
			},
		)
		.unwrap();

		let queue = queues.next().unwrap();

		let (mut swapchain, images) = {
			let caps = physical_device
				.surface_capabilities(&surface, Default::default())
				.unwrap();
			let usage = caps.supported_usage_flags;
			let alpha = caps.supported_composite_alpha.iter().next().unwrap();
			let image_format = Some(
				physical_device
					.surface_formats(&surface, Default::default())
					.unwrap()[0]
					.0,
			);

			Swapchain::new(
				device.clone(),
				surface.clone(),
				SwapchainCreateInfo {
					min_image_count: caps.min_image_count,
					image_format,
					image_extent: surface.window().inner_size().into(),
					image_usage: usage,
					composite_alpha: alpha,
					..Default::default()
				},
			)
			.unwrap()
		};

		let render_pass = vulkano::single_pass_renderpass!(
			device.clone(),
			attachments: {
				color: {
					load: Clear,
					store: Store,
					format: swapchain.image_format(),
					samples: 1,
				}
			},
			pass: {
				color: [color],
				depth_stencil: {}
			}
		)
		.unwrap();

		let mut viewport = Viewport {
			origin: [0.0, 0.0],
			dimensions: [0.0, 0.0],
			depth_range: 0.0..1.0,
		};

		let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

		let mut recreate_swapchain = false;

		let mut previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);

		previous_frame_end.as_mut().take().unwrap().cleanup_finished();

		(WindowData { surface: surface }, event_loop)
	}

	pub fn loopd(
		self,
		event_loop: winit::event_loop::EventLoop<()>,
		draw: for<'r> fn(&'r WindowData),
	) {
		event_loop.run(move |event, _, control_flow| {
			let event_ref = &event;

			if let winit::event::Event::WindowEvent { event, .. } = event_ref {
				match event {
					winit::event::WindowEvent::CloseRequested
					| winit::event::WindowEvent::KeyboardInput {
						input:
							winit::event::KeyboardInput {
								virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						*control_flow = winit::event_loop::ControlFlow::Exit;
						return;
					}
					_ => (),
				}
			}

			if let winit::event::Event::RedrawRequested { .. } = event_ref {
				draw(&self);
			}
		});
	}
}

fn window_size_dependent_setup(
    images: &[std::sync::Arc<SwapchainImage<Window>>],
    render_pass: std::sync::Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<std::sync::Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}


pub fn run() {
	let (data, eloop) = WindowData::new();

	data.loopd(eloop, crate::ui::draw);
}
