- https://github.com/rust-windowing/winit/tree/master/examples

<hr>

# (winit)Window handling library
-  Window handling library in pure Rust
  - https://docs.rs/winit/latest/winit/
    - https://github.com/rust-windowing/winit

# winit 0.30.5버젼 해결하는 코드 모음
- https://github.com/rust-windowing/winit/discussions/3667
- https://github.com/rbxb/wgpu_test
- The trait `raw_window_handle::HasRawDisplayHandle` is not implemented for `Window` [closed]
  - https://stackoverflow.com/questions/77373416/the-trait-raw-window-handlehasrawdisplayhandle-is-not-implemented-for-windo
  - `Cargo.toml` 추가하면 끝
    - `winit = { version = "0.30.5", features = ["rwh_05"]}`

# window.rs 여기를 참고하자
- https://github.com/rust-windowing/winit/blob/master/examples/window.rs#L43-L72

```rs
fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(web_platform)]
    console_error_panic_hook::set_once();

    tracing::init();

    let event_loop = EventLoop::new()?;
    let (sender, receiver) = mpsc::channel();

    // Wire the user event from another thread.
    #[cfg(not(web_platform))]
    {
        let event_loop_proxy = event_loop.create_proxy();
        let sender = sender.clone();
        std::thread::spawn(move || {
            // Wake up the `event_loop` once every second and dispatch a custom event
            // from a different thread.
            info!("Starting to send user event every second");
            loop {
                let _ = sender.send(Action::Message);
                event_loop_proxy.wake_up();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    let app = Application::new(&event_loop, receiver, sender);
    Ok(event_loop.run_app(app)?)
}
```
# Result

```bash

```

# Result

```bash

```

