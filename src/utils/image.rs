use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use std::path::Path;

pub fn load_scene_image(image_path: &str) -> Result<Box<dyn StatefulProtocol>, String> {
    if !Path::new(image_path).exists() {
        return Err(format!("Image file not found: {}", image_path));
    }

    // Use image 0.24 API
    let dyn_img = image::open(image_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
    ;

    let mut picker = Picker::new((8, 12));

    // picker must be mutable
    let protocol = picker.new_resize_protocol(dyn_img);

    Ok(protocol)
}
