use drawille::Canvas;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::path::Path;

pub fn load_image_as_braille(
    path: &str,
    max_width: u32,
    max_height: u32,
) -> Result<String, String> {
    // Check if file exists
    if !Path::new(path).exists() {
        return Err(format!("Image file not found: {}", path));
    }

    // Load the image
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    image_to_braille(&img, max_width, max_height)
}

pub fn image_to_braille(
    img: &DynamicImage,
    max_width: u32,
    max_height: u32,
) -> Result<String, String> {
    let (orig_width, orig_height) = img.dimensions();

    // Calculate target dimensions maintaining aspect ratio
    // Braille characters are 2 pixels wide and 4 pixels tall
    let aspect = orig_width as f32 / orig_height as f32;
    let max_aspect = (max_width * 2) as f32 / (max_height * 4) as f32;

    let (target_width, target_height) = if aspect > max_aspect {
        let w = max_width * 2;
        let h = (w as f32 / aspect) as u32;
        (w, h)
    } else {
        let h = max_height * 4;
        let w = (h as f32 * aspect) as u32;
        (w, h)
    };

    // Resize the image
    let resized = img.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Convert to grayscale
    let gray = resized.to_luma8();

    // Create drawille canvas
    let mut canvas = Canvas::new(target_width, target_height);

    // Convert pixels to braille dots
    let threshold = 128u8;

    for (x, y, pixel) in gray.enumerate_pixels() {
        if pixel[0] < threshold {
            canvas.set(x as u32, y as u32);
        }
    }

    // Get the braille string using drawille's frame method
    Ok(canvas.frame())
}

/// Load and cache braille art for a scene
pub fn load_scene_image(image_path: &str) -> String {
    // Use reasonable default dimensions
    match load_image_as_braille(image_path, 40, 20) {
        Ok(art) => {
            eprintln!("Successfully loaded image: {}", image_path);
            art
        }
        Err(e) => {
            eprintln!("Warning: Failed to load image '{}': {}", image_path, e);
            format!("[Image not available: {}]", image_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() {
        if Path::new("test_image.png").exists() {
            let result = load_image_as_braille("test_image.png", 40, 20);
            assert!(result.is_ok());
            println!("{}", result.unwrap());
        }
    }
}
