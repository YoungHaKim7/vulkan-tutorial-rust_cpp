#include <glslang/Public/ResourceLimits.h>
#include <glslang/Public/ShaderLang.h>
#include <glslang/SPIRV/GlslangToSpv.h>
#include <iostream>
#include <string>
#include <vector>

// A basic fragment shader string to compile
const char *fragmentShaderSource = R"(
    #version 450
    layout(location = 0) out vec4 fragColor;
    void main() {
        fragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
)";

int main() {
    // 1. Initialize the glslang library process
    glslang::InitializeProcess();

    // 2. Map the shader stage (Fragment Shader)
    EShLanguage stage = EShLangFragment;
    glslang::TShader shader(stage);

    // 3. Set up the source code strings
    const char *shaderStrings[1] = {fragmentShaderSource};
    shader.setStrings(shaderStrings, 1);

    // 4. Configure target environment (Vulkan 1.2 / SPIR-V 1.5)
    int clientInputSemanticsVersion = 100; // Map to GLSL versioning rules
    glslang::EShTargetClientVersion vulkanVersion =
        glslang::EShTargetVulkan_1_2;
    glslang::EShTargetLanguageVersion spirvVersion = glslang::EShTargetSpv_1_5;

    shader.setEnvInput(glslang::EShSourceGlsl, stage, glslang::EShClientVulkan,
                       clientInputSemanticsVersion);
    shader.setEnvClient(glslang::EShClientVulkan, vulkanVersion);
    shader.setEnvTarget(glslang::EShTargetSpv, spirvVersion);

    // 5. Use the standard resource limits (a zero-initialized TBuiltInResource
    // would reject any shader, since every limit would be 0)
    const TBuiltInResource *resources = GetDefaultResources();

    // 6. Preprocess and parse the shader
    EShMessages messages =
        (EShMessages)(EShMsgDefault | EShMsgSpvRules | EShMsgVulkanRules);

    if (!shader.parse(resources, 100, false, messages)) {
        std::cerr << "GLSL Parsing Failed:\n"
                  << shader.getInfoLog() << "\n"
                  << shader.getInfoDebugLog() << std::endl;
        glslang::FinalizeProcess();
        return 1;
    }

    // 7. Link the shader program
    glslang::TProgram program;
    program.addShader(&shader);

    if (!program.link(messages)) {
        std::cerr << "Shader Linking Failed:\n"
                  << program.getInfoLog() << "\n"
                  << program.getInfoDebugLog() << std::endl;
        glslang::FinalizeProcess();
        return 1;
    }

    // 8. Generate SPIR-V binary payload
    std::vector<unsigned int> spirvBinary;
    spv::SpvBuildLogger logger;
    glslang::SpvOptions spvOptions;

    glslang::GlslangToSpv(*program.getIntermediate(stage), spirvBinary, &logger,
                          &spvOptions);

    std::cout << "Success! Compiled SPIR-V word count: " << spirvBinary.size()
              << std::endl;

    // 9. Clean up resources
    glslang::FinalizeProcess();
    return 0;
}
