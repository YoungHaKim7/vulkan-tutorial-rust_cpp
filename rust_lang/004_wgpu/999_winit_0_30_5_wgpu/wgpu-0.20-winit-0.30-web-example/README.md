# 출처

- https://github.com/erer1243/wgpu-0.20-winit-0.30-web-example

- Winit 0.30.0 diverged in compatibility with Wgpu on the web. Winit now requires windows to be created during the event loop, which means any async Wgpu initialization must also happen during the event loop. This is a problem because, although the event loop exists in an async context on the web, Winit does not provide access to it. A workaround is to queue a future with wasm_bindgen_futures::spawn_local that will asynchronously construct a graphical context and send it to our event loop when it's ready through Winit's user event interface. This example implements Wgpu's hello-triangle using this hack.

