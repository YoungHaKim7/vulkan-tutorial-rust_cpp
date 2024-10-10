use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

pub struct AppWindow {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App resumed");
        if self.window.is_none() {
            println!("Creating window and renderer");

            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("GY winit")
                            .with_inner_size(PhysicalSize {
                                width: WIDTH,
                                height: HEIGHT,
                            }),
                    )
                    .unwrap(),
            );
            self.window = Some(window.clone());

            // let mut state = pollster::block_on(RenderState::new(window.clone()));
            // state.create_pipelines();
            // state.randomize();
            // self.state = Some(state);
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App exiting");
        // shut down
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        let _ = (event_loop, cause);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // if event == WindowEvent::Destroyed && self.window == Some(window_id) {
        if event == WindowEvent::Destroyed {
            println!(
                "--------------------------------------------------------- Window {:?} Destroyed",
                self.window
            );
            self.window = None;
            event_loop.exit();
            return;
        }

        let window = match self.window.as_mut() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                println!(
                    "--------------------------------------------------------- Window {:?} \
                         CloseRequested",
                    self.window
                );
                // fill::cleanup_window(window.as_ref());
                self.window = None;
            }
            WindowEvent::RedrawRequested => {
                // todo()
                // fill::fill_window(window.as_ref());
            }
            _ => (),
        }
    }
}
