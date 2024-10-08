// use winit::event_loop::EventLoop;

// const WIDTH: u32 = 800;
// const HEIGHT: u32 = 600;

// struct HelloTriangleApplication {
//     events_loop: EventLoop,
// }

// impl HelloTriangleApplication {
//     pub fn initialize() -> Self {
//         Self {}
//     }

//     fn main_loop(&mut self) {}

//     pub fn run() -> Self {
//         let events_loop = Self::init_window();

//         Self { events_loop }
//     }

//     fn init_window() -> EventLoop {
//         let events_loop = EventLoop::new();
//         let window = WindowBuilder::new().build(&events_loop).unwrap();

//         events_loop.run(move |event, _, control_flow| {
//             *control_flow = winit::event_loop::ControlFlow::Wait;

//             match event {
//                 winit::event::Event::WindowEvent {
//                     event: winit::event::WindowEvent::CloseRequested,
//                     ..
//                 } => *control_flow = winit::event_loop::ControlFlow::Exit,
//                 _ => (),
//             }
//         });
//     }
// }

// fn main() {
//     let mut app = HelloTriangleApplication::initialize();
//     app.main_loop();
// }

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}
