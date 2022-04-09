extern crate vulkano;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	pub position: [f32; 2],
	pub uv: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, uv);

pub struct WindowData {
	// Surface to draw onto
	surface: winit::window::Window,
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

		let event_loop = winit::event_loop::EventLoop::new(); // ignore this for now
		let surface = winit::window::WindowBuilder::new()
			.build(&event_loop)
   			.unwrap();

		let (physical_device, queue_family) = vulkano::device::physical::PhysicalDevice::enumerate(&instance)
			.filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
			.filter_map(|p| {
				p.queue_families()
					//.find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
					.find(|&q| q.supports_graphics())
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
				enabled_extensions: physical_device.required_extensions().union(&device_extensions),

				queue_create_infos: vec![vulkano::device::QueueCreateInfo::family(queue_family)],
				..Default::default()
			},
		)
		.unwrap();

		let queue = queues.next().unwrap();

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

pub fn run() {
	let (data, eloop) = WindowData::new();

	data.loopd(eloop, crate::ui::draw);
}
