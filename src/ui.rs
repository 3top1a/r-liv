extern crate vulkano;

use crate::window;

use std::sync::Arc;

use egui::{ScrollArea, TextEdit, TextStyle};
use egui_winit_vulkano::Gui;
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

pub fn draw(data: &mut window::WindowData, gui: &mut Gui) {
	// Render GUI
	if data.recreate_swapchain {
		data.recreate_swapchain();
	}
	// Acquire next image in the swapchain and our image num index
	let (image_num, suboptimal, acquire_future) =
	match swapchain::acquire_next_image(data.swap_chain.clone(), None) {
		Ok(r) => r,
		Err(AcquireError::OutOfDate) => {
			data.recreate_swapchain = true;
			return;
		}
		Err(e) => panic!("Failed to acquire next image: {:?}", e),
	};
	if suboptimal {
		data.recreate_swapchain = true;
	};
	// Render GUI
	let future = data.previous_frame_end.take().unwrap().join(acquire_future);
	let after_future = gui.draw_on_image(future, data.final_images[image_num].clone());

	// Finish render
	data.finish(after_future, image_num);
}
