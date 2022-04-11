extern crate vulkano;

use crate::window;

use std::sync::Arc;

use vulkano::{
	device::{
		physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Features, Queue,
		QueueCreateInfo,
	},
	image::{view::ImageView, ImageUsage, SwapchainImage},
	instance::{Instance, InstanceCreateInfo, InstanceExtensions},
	swapchain,
	swapchain::{
		AcquireError, PresentMode, Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
	},
	sync,
	sync::{FlushError, GpuFuture},
	Version,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

pub fn draw(data: &window::WindowData) {
	// Render GUI
	/*let (image_num, suboptimal, acquire_future) =
		match swapchain::acquire_next_image(data.swap_chain.clone(), None) {
			Ok(r) => r,
			Err(AcquireError::OutOfDate) => {
				return;
			}
			Err(e) => panic!("Failed to acquire next image: {:?}", e),
		};

	let future = data.previous_frame_end.take().unwrap().join(acquire_future);
	let after_future = data
		.gui
		.draw_on_image(future, data.final_images[image_num].clone());

	// Finish render
	let future = after_future
		.then_swapchain_present(data.queue.clone(), data.swap_chain.clone(), image_num)
		.then_signal_fence_and_flush();
	match future {
		Ok(future) => {
			// A hack to prevent OutOfMemory error on Nvidia :(
			// https://github.com/vulkano-rs/vulkano/issues/627
			match future.wait(None) {
				Ok(x) => x,
				Err(err) => println!("err: {:?}", err),
			}
			data.previous_frame_end = Some(future.boxed());
		}
		Err(FlushError::OutOfDate) => {
			data.recreate_swapchain = true;
			data.previous_frame_end = Some(sync::now(data.device.clone()).boxed());
		}
		Err(e) => {
			println!("Failed to flush future: {:?}", e);
			data.previous_frame_end = Some(sync::now(data.device.clone()).boxed());
		}
	}*/
}

pub fn run(filename: String) {
	let (data, eloop) = WindowData::new(filename);

	data.loopd(eloop, draw);
}
