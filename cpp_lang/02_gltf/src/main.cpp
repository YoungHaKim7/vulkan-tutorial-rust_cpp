#include <iostream>
#include <string>
#include <algorithm>
#include <stdexcept>
#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "tiny_gltf.h"

void loadModel(const std::string& modelPath) {
    // Create a tinygltf loader
    tinygltf::Model gltfModel;
    tinygltf::TinyGLTF loader;
    std::string err, warn;

    // Detect file extension to determine which loader to use
    bool ret = false;
    size_t dotPos = modelPath.find_last_of(".");
    if (dotPos == std::string::npos) {
        throw std::runtime_error("No file extension found in path: " + modelPath);
    }
    std::string extension = modelPath.substr(dotPos + 1);
    std::transform(extension.begin(), extension.end(), extension.begin(),
                  [](unsigned char c) { return std::tolower(c); });

    if (extension == "glb") {
        ret = loader.LoadBinaryFromFile(&gltfModel, &err, &warn, modelPath);
    } else if (extension == "gltf") {
        ret = loader.LoadASCIIFromFile(&gltfModel, &err, &warn, modelPath);
    } else {
        err = "Unsupported file extension: " + extension + ". Expected .gltf or .glb";
    }

    // Handle errors and warnings
    if (!warn.empty()) {
        std::cout << "glTF warning: " << warn << std::endl;
    }
    if (!err.empty()) {
        std::cout << "glTF error: " << err << std::endl;
    }
    if (!ret) {
        throw std::runtime_error("Failed to load glTF model");
    }

    // Clear existing model data
    // model = Model();  // TODO: Define Model type and instance

    // Process the loaded data (covered in the following sections)
}

int main() {
    try {
        loadModel("model.gltf");
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
