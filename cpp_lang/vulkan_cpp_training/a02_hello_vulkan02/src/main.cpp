#include <iostream>
#include <vulkan/vulkan.h>

int main()
{
    VkInstanceCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;

    VkInstance instance;
    VkResult result = vkCreateInstance(&createInfo, nullptr, &instance);
    if (result != VK_SUCCESS) {
        std::cerr << "Failed to create Vulkan instance!" << std::endl;
        return 1;
    }

    // >> 입력
    std::cout << "Hello Vulkan! Instance created successfully." << std::endl;

    // free 메모리 해지
    vkDestroyInstance(instance, nullptr);
    return 0;
}
