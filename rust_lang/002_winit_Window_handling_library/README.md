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
