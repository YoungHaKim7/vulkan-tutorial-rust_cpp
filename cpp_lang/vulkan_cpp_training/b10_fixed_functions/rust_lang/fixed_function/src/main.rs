struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    // Some(값)   // Some(1)   , None( C언어에서 null)
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    // graphics_family와 present_family 둘 다 데이터(Some)가 있으면 true
    fn has_value(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

fn main() {
    let complete = QueueFamilyIndices {
        graphics_family: Some(0),
        present_family: Some(0),
    };
    println!("complete: {}", complete.has_value()); // true

    let incomplete = QueueFamilyIndices {
        graphics_family: Some(0),
        present_family: None,
    };
    println!("incomplete: {}", incomplete.has_value()); // false
}
