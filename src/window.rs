extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

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

pub struct WindowData {
	#[allow(dead_code)]
	pub instance: Arc<Instance>,
	pub device: Arc<Device>,
	pub surface: Arc<Surface<Window>>,
	pub queue: Arc<Queue>,
	pub swap_chain: Arc<Swapchain<Window>>,
	pub final_images: Vec<Arc<ImageView<SwapchainImage<Window>>>>,
	pub recreate_swapchain: bool,
	pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

// Quite a bit of this code has been copied from https://github.com/hakolao/egui_winit_vulkano/blob/master/examples/minimal.rs
// Which is copyrighted under MIT

impl WindowData {
	pub fn new(filename: String) -> (WindowData, winit::event_loop::EventLoop<()>) {
		let event_loop = EventLoop::new();

		// TODO Detect size from image from filename
		let window_size = [1280u16, 720u16];

		// TODO Detect name from image from filename
		let window_title = format!("R-Liv | {}", filename);

		//* Create renderer and all
		// Add instance extensions based on needs
		let instance_extensions = InstanceExtensions {
			..vulkano_win::required_extensions()
		};
		// Create instance
		let instance = Instance::new(InstanceCreateInfo {
			application_version: Version::V1_2,
			enabled_extensions: instance_extensions,
			..Default::default()
		})
		.expect("Failed to create instance");

		// Get most performant device (physical)
		let physical = PhysicalDevice::enumerate(&instance)
			.fold(None, |acc, val| {
				if acc.is_none() {
					Some(val)
				} else if acc.unwrap().properties().max_compute_shared_memory_size
					>= val.properties().max_compute_shared_memory_size
				{
					acc
				} else {
					Some(val)
				}
			})
			.expect("No physical device found");
		println!("Using device {}", physical.properties().device_name);

		// Create rendering surface along with window
		let surface = WindowBuilder::new()
			.with_inner_size(winit::dpi::LogicalSize::new(window_size[0], window_size[1]))
			.with_title(window_title)
			.build_vk_surface(&event_loop, instance.clone())
			.expect("Failed to create vulkan surface & window");

		// Create device
		let (device, queue) = crate::utils::create_device(physical, surface.clone());
		// Create swap chain & frame(s) to which we'll render
		let (swap_chain, images) =
			crate::utils::create_swap_chain(surface.clone(), physical, device.clone());
		let previous_frame_end = Some(sync::now(device.clone()).boxed());

		(
			WindowData {
				instance,
				device,
				surface,
				queue,
				swap_chain,
				final_images: images,
				previous_frame_end,
				recreate_swapchain: false,
			},
			event_loop,
		)
	}

	pub fn loopd(
		mut self,
		event_loop: winit::event_loop::EventLoop<()>,
		draw: for<'r, 's> fn(&'r mut WindowData, &'s mut egui_winit_vulkano::Gui),
	) {
		let mut gui = Gui::new(self.surface.clone(), self.queue.clone(), false);

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
				draw(&mut self, &mut gui);
			}
		});

		// Create gui state (pass anything your state requires)
		/*event_loop.run(move |event, _, control_flow| {
			match event {
				Event::WindowEvent { event, window_id }
					if window_id == renderer.surface().window().id() =>
				{
					// Update Egui integration so the UI works!
					let _pass_events_to_game = !gui.update(&event);
					match event {
						WindowEvent::Resized(_) => {
							renderer.resize();
						}
						WindowEvent::ScaleFactorChanged { .. } => {
							renderer.resize();
						}
						WindowEvent::CloseRequested => {
							*control_flow = ControlFlow::Exit;
						}
						_ => (),
					}
				}
				Event::RedrawRequested(window_id) if window_id == window_id => {
					// Set immediate UI in redraw here
					gui.immediate_ui(|gui| {
						let ctx = gui.context();
						egui::CentralPanel::default().show(&ctx, |ui| {
							ui.vertical_centered(|ui| {
								ui.add(egui::widgets::Label::new("Hi there!"));
							});
							ui.separator();
							ui.columns(2, |columns| {
								ScrollArea::vertical().id_source("source").show(
									&mut columns[0],
									|ui| {
										ui.add(
											TextEdit::multiline(&mut "asdasdasd".to_owned())
												.font(TextStyle::Monospace),
										);
									},
								);
							});
						});
					});
					// Render UI
					renderer.render(&mut gui);
				}
				Event::MainEventsCleared => {
					renderer.surface().window().request_redraw();
				}
				_ => (),
			}
		});*/
	}

	pub fn recreate_swapchain(&mut self) {
        let dimensions: [u32; 2] = self.surface.window().inner_size().into();
        let (new_swapchain, new_images) = match self.swap_chain.recreate(SwapchainCreateInfo {
            image_extent: dimensions,
            ..self.swap_chain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
        self.swap_chain = new_swapchain;
        let new_images = new_images
            .into_iter()
            .map(|image| ImageView::new_default(image).unwrap())
            .collect::<Vec<_>>();
        self.final_images = new_images;

        self.recreate_swapchain = false;
    }

	pub fn finish(&mut self, after_future: Box<dyn GpuFuture>, image_num: usize) {
        let future = after_future
            .then_swapchain_present(self.queue.clone(), self.swap_chain.clone(), image_num)
            .then_signal_fence_and_flush();
        match future {
            Ok(future) => {
                // A hack to prevent OutOfMemory error on Nvidia :(
                // https://github.com/vulkano-rs/vulkano/issues/627
                match future.wait(None) {
                    Ok(x) => x,
                    Err(err) => println!("err: {:?}", err),
                }
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
}

pub fn run(filename: String) {
	let (data, eloop) = WindowData::new(filename);

	data.loopd(eloop, crate::ui::draw);
}
