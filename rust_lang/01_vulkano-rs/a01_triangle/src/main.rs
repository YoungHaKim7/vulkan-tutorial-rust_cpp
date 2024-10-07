use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};

fn main() {
    let library = VulkanLibrary::new().expect("Failed to create Vulkan library");

    let instance_create_info = InstanceCreateInfo {
        enabled_extensions: InstanceExtensions {
            ..InstanceExtensions::default()
        },
        ..Default::default()
    };

    let instance = Instance::new(library, instance_create_info).expect("Failed to create instance");

    for physical_device in instance
        .enumerate_physical_devices()
        .expect("Failed to enumerate physical devices")
    {
        println!(
            "Available device : {}",
            physical_device.properties().device_name
        )
    }
}
