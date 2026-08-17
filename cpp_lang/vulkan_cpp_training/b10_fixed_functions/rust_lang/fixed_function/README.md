# Result


```bash
complete: true
incomplete: false

```

# C++ 코드 러스트 코드로 변환
- Compiles and runs — `complete: true`, `incomplete: false`. Key fixes from your draft:

- **`&self` parameter** — the function checks the struct instance's fields, so it needs `fn is_complete(&self)`. Without it, the compiler errors with "expected `self`" for an impl method.
- **`is_some()`** — this is the idiomatic "is there data?" check on `Option<T>` (`true` for `Some(x)`, `false` for `None`). There's no `is_empty()` on `Option`.
- **`false`/`true` are lowercase** in Rust, unlike C's `true`/`FALSE` macros.
- Renamed `QueueFmilyIndices` → `QueueFamilyIndices` (typo — worth fixing now before it spreads through later chapters).

The `graphics_family.is_some() && present_family.is_some()` shape is exactly what the tutorial's `find_queue_families` uses later: it loops over `queue_family_properties`, stores indices into the `Option`s, and calls `is_complete()` to know when it can stop.

