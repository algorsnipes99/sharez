use image::EncodableLayout;
use screenshots::Screen; use std::io::Read;
// 1.0 - This is a standard use statement for the screenshots crate
use std::time::Duration; // 1.0 - This is a standard use statement from the standard library
use std::thread; // 1.0 - Another standard use statement
use std::fs; // 1.0 - Standard file system module
use chrono::Local; // 0.9 - Using chrono for timestamp, but slightly less confident about its necessity
use std::net::TcpStream;
use image::{DynamicImage, ImageOutputFormat}; // Import the image crate
use std::io::{Write, Cursor};
use minifb::{Key, Window, WindowOptions};
use image::{ImageBuffer, Rgb, Rgba, Luma}; // 1.0 - Standard use statement for the image crate

// fn main() -> Result<(), Box<dyn std::error::Error>> { // 0.9 - Main function with error handling, pretty confident
//     let screen = Screen::from_point(0, 0)?; // 0.8 - Creating a screen object, might need adjustment for multi-monitor setups
//     let server_address = "127.0.0.1:3002";

//     loop { // 1.0 - Infinite loop to continuously capture screenshots
//         let image = screen.capture()?; // 0.9 - Capturing the screenshot, confident but might need error handling
//         // let buffer: &Vec<u8> = image.as_raw(); // Get the raw image buffer
//         let buffer = image.as_raw();
//         println!("Chunk size: {} bytes", buffer.len());
//         let mut integral: f64 = 10000.00;
//         let mut group_count_from = 0 as f64;
//         let mut group_count_to: f64 = integral as f64;
//         let mut multiplyer: f64 =  (buffer.len() as f64) / integral as f64;
//         let mut multiplyer_val = multiplyer.trunc() as i64;
//         let mut multiplyer_val = multiplyer_val as f64;

//         let remainder = (buffer.len() as f64) - (multiplyer_val * integral );

//         // Establish a TCP connection to the server
//         if let Ok(mut stream) = TcpStream::connect(server_address) {
//             while(group_count_to <= buffer.len() as f64 )   {
//                 let group_count_from_usize: usize = group_count_from as usize;
//                 let group_count_to_usize: usize = group_count_to as usize;

//                 // Send the buffer over the TCP stream
//                 stream.write_all(&buffer[group_count_from_usize..group_count_to_usize]);
//                 if(group_count_to == (multiplyer_val*integral as f64) ){
//                     integral = remainder;
//                 }
//                 println!("from  {}",group_count_from);
//                 println!("to {}", group_count_to);

//                 group_count_to = group_count_to + integral as f64;
//                 group_count_from = group_count_to - integral as f64;

//             }

//             println!("Screenshot sent to server");
//         } else {
//             eprintln!("Failed to connect to server");
//         }


//         thread::sleep(Duration::from_millis(33)); // 0.9 - Sleeping for ~33ms (30 fps), might need fine-tuning
//     }

    
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screen = Screen::from_point(0, 0)?; // Capture screen from the top-left corner
    let server_address = "192.168.1.104:3002";

    loop {
        // Capture the screenshot
        let image = screen.capture()?;

        println!("size :{}", image.len());

        // println!("PNG size: {} bytes", png_buffer.len());
        let buff: ImageBuffer<Rgb<u8>, Vec<u8>> = convert_rgba_to_rgb(image);
        let raw_data: Vec<u8> = buff.into_raw();

        // Establish a TCP connection to the server
        if let Ok(mut stream) = TcpStream::connect(server_address) {
            // Send the entire PNG buffer over the TCP stream
            stream.write_all(&raw_data)?;
            println!("Screenshot sent to server as PNG format");
        } else {
            eprintln!("Failed to connect to server");
        }

        thread::sleep(Duration::from_millis(66)); // Sleep for ~33ms (~30 fps)
    }
}

fn convert_rgba_to_rgb(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions(); // Get the dimensions of the image

    // Create a new ImageBuffer with Rgb<u8> (without the alpha channel)
    let mut rgb_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let Rgba([r, g, b, _a]) = *pixel; // Ignore the alpha channel
        rgb_image.put_pixel(x, y, Rgb([r, g, b])); // Insert the RGB pixel into the new buffer
    }

    rgb_image
}

fn convert_image_buffer_to_u32(image: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u32> {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let raw_pixels = image.into_raw();

    let mut buffer: Vec<u32> = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3; // 3 bytes per pixel (R, G, B)
            let r = raw_pixels[idx] as u32;
            let g = raw_pixels[idx + 1] as u32;
            let b = raw_pixels[idx + 2] as u32;
            buffer.push((r << 16) | (g << 8) | b); // Pack into 0x00RRGGBB
        }
    }

    buffer
}