use egui_winit_vulkano::Gui;
use winit::{
	event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow,},
};

struct WindowData {
	// image
	// uniform
}

pub fn main(filename: String) {
	// Create window
	let window_size = [1280, 720];
	//let window_size =
	// utils::get_size.unwrap_or([1280, 720]);
	let window_title = format!(
		"R-Liv | {}",
		crate::utils::get_file_stem(filename)
	);

	let (mut renderer, event_loop) = crate::renderer::SimpleGuiRenderer::new(
		window_size,
		window_title.as_str()
	);
	let mut gui = Gui::new_with_subpass(renderer.surface(), renderer.queue(), renderer.gui_pass());

	// let mut 

	event_loop.run(move |event, _, control_flow| {
		match event {
			// Redraw
			Event::RedrawRequested(window_id) if window_id == window_id => {
				// Render GUI
				gui.immediate_ui(|gui| {
					let ctx = gui.context();
					/*egui::SidePanel::right("Sidepanel").show(&ctx, |ui| {
						ui.vertical_centered(|ui| {
							ui.add(egui::widgets::Label::new("Hello there"));
						});
					});*/
				});
				renderer.render(&mut gui);
			}
			Event::WindowEvent { event, window_id }
				if window_id == renderer.surface().window().id() =>
			{
				// Update Egui integration so the UI works!
				let _pass_events_to_game = !gui.update(&event);
				match event {
					// Resize
					WindowEvent::Resized(_) => {
						renderer.resize();
					}
					WindowEvent::ScaleFactorChanged { .. } => {
						renderer.resize();
					}

					// Close
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

					// Move

					// Image change
					WindowEvent::KeyboardInput
					{
						input:
							KeyboardInput {
								virtual_keycode: Some(VirtualKeyCode::Left),
								..
							},
						..

					} =>
					{
						// IDK
					}
					_ => (),
				}
			}
			Event::MainEventsCleared => {
				renderer.surface().window().request_redraw();
			}
			_ => (),
		}
	});
}
