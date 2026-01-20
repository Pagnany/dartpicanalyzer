use chrono::prelude::*;
use image::{GenericImageView, ImageReader, Rgba, RgbaImage};

fn main() {
    println!("Starting image processing...");
    let start_time = Local::now();

    // Load the input image
    let img = ImageReader::open("input/dart2.png")
        .unwrap()
        .decode()
        .unwrap();
    //let img = ImageReader::open("input/dart.jpg").unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    println!("Image dimensions: {}x{}", width, height);

    // Create a new image to store only red pixels
    let mut red_only_img = RgbaImage::new(width, height);
    let mut red_pixel_count = 0;

    // Create a new image to store only green pixels
    let mut green_only_img = RgbaImage::new(width, height);
    let mut green_pixel_count = 0;

    // Create a new image to store both red and green pixels
    let mut red_green_img = RgbaImage::new(width, height);

    // Iterate through all pixels
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let (r, g, b, _a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

            // Use relative thresholds: a color is dominant if it's at least 25% stronger than the others
            // This works for both bright and dark colors
            let is_red = r > 30 && r.saturating_sub(g) > (g / 4) && r.saturating_sub(b) > (b / 4);
            let is_green = g > 30 && g.saturating_sub(r) > (r / 4) && g.saturating_sub(b) > (b / 4);

            // Check if pixel is predominantly red
            // A pixel is considered "red" if red channel is significantly higher than green and blue
            if is_red {
                // Keep the red pixel
                red_only_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                red_pixel_count += 1;
            } else {
                // Make non-red pixels transparent
                red_only_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }

            // Check if pixel is predominantly green
            // A pixel is considered "green" if green channel is significantly higher than red and blue
            if is_green {
                // Keep the green pixel
                green_only_img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
                green_pixel_count += 1;
            } else {
                // Make non-green pixels transparent
                green_only_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }

            // For the combined image, keep pixel if it's red or green
            if is_red || is_green {
                if is_red {
                    red_green_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                } else {
                    red_green_img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
                }
            } else {
                red_green_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    println!("Found {} red pixels", red_pixel_count);
    println!("Found {} green pixels", green_pixel_count);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("output").unwrap();

    // Save the output image
    red_only_img.save("output/red_pixels_only.png").unwrap();
    println!("Saved output image as 'red_pixels_only.png'");

    green_only_img.save("output/green_pixels_only.png").unwrap();
    println!("Saved output image as 'green_pixels_only.png'");

    red_green_img
        .save("output/red_and_green_pixels.png")
        .unwrap();
    println!("Saved output image as 'red_and_green_pixels.png'");

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("Processing took: {:?}", duration);
}
