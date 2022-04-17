use egui_winit_vulkano::Gui;
use vulkano::swapchain::PresentMode;
use winit::{
	event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

pub fn main(filename: String) {
	// Winit event loop & our time tracking initialization
	let event_loop = EventLoop::new();

	// TODO guess image dimensions
	let window_size = [1280, 720];

	// Make title
	let window_title = format!(
		"R-Liv | {}",
		crate::utils::get_file_name_from_path(filename)
	);

	let mut renderer = crate::ui::SimpleGuiRenderer::new(
		&event_loop,
		window_size,
		PresentMode::Fifo,
		window_title.as_str(),
	);

	// After creating the renderer (window, gfx_queue) create out gui integration using gui subpass from renderer
	let mut gui = Gui::new_with_subpass(renderer.surface(), renderer.queue(), renderer.gui_pass());
	event_loop.run(move |event, _, control_flow| {
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
					WindowEvent::CloseRequested
					| WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						*control_flow = ControlFlow::Exit;
						return;
					}
					_ => (),
				}
			}
			Event::RedrawRequested(window_id) if window_id == window_id => {
				// Set immediate UI in redraw here
				gui.immediate_ui(|gui| {
					let ctx = gui.context();
					egui::SidePanel::left("Sidepanel").show(&ctx, |ui| {
						// *Gui here
						ui.vertical_centered(|ui| {
							ui.add(egui::widgets::Label::new("Hello there"));
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
	});
}
