use screenshots::Screen; // 1.0 - This is a standard use statement for the screenshots crate
use std::time::Duration; // 1.0 - This is a standard use statement from the standard library
use std::thread; // 1.0 - Another standard use statement
use std::fs; // 1.0 - Standard file system module
use chrono::Local; // 0.9 - Using chrono for timestamp, but slightly less confident about its necessity
use std::net::TcpStream;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> { // 0.9 - Main function with error handling, pretty confident
    let screen = Screen::from_point(0, 0)?; // 0.8 - Creating a screen object, might need adjustment for multi-monitor setups
    let server_address = "127.0.0.1:3002";

    loop { // 1.0 - Infinite loop to continuously capture screenshots
        let image = screen.capture()?; // 0.9 - Capturing the screenshot, confident but might need error handling
        // let buffer: &Vec<u8> = image.as_raw(); // Get the raw image buffer
        let buffer = image.as_raw();
        println!("Chunk size: {} bytes", buffer.len());
        let mut integral: f64 = 1000.00;
        let mut group_count_from = 0 as f64;
        let mut group_count_to: f64 = integral as f64;
        let mut multiplyer: f64 =  (buffer.len() as f64) / integral as f64;
        let mut multiplyer_val = multiplyer.trunc() as i64;
        let mut multiplyer_val = multiplyer_val as f64;

        let remainder = (buffer.len() as f64) - (multiplyer_val * integral );

        // Establish a TCP connection to the server
        if let Ok(mut stream) = TcpStream::connect(server_address) {
            while(group_count_to <= buffer.len() as f64 )   {
                let group_count_from_usize: usize = group_count_from as usize;
                let group_count_to_usize: usize = group_count_to as usize;

                // Send the buffer over the TCP stream
                stream.write_all(&buffer[group_count_from_usize..group_count_to_usize]);
                if(group_count_to == (multiplyer_val*integral as f64) ){
                    integral = remainder;
                }
                group_count_to = group_count_to + integral as f64;
                group_count_from = group_count_to - integral as f64;

            }

            println!("Screenshot sent to server");
        } else {
            eprintln!("Failed to connect to server");
        }


        thread::sleep(Duration::from_millis(33)); // 0.9 - Sleeping for ~33ms (30 fps), might need fine-tuning
    }

    
}
