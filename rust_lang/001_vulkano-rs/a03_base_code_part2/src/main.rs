use std::error::Error;

use vulkano::{
    device::DeviceExtensions,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};
use winit::event_loop::EventLoop;

fn required_extensions(event_loop: &EventLoop<()>) -> InstanceExtensions {
    InstanceExtensions {
        khr_surface: true,
        ..InstanceExtensions::empty()
    }
}

fn main() -> Result<(), impl Error> {
    let event_loop = EventLoop::new().unwrap();
    let library = VulkanLibrary::new().unwrap();
    let required_extensions = required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
    })
}
