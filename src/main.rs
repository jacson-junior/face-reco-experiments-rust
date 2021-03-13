mod bindings;

use minifb::{Key, Window, WindowOptions};
use openpnp_capture::{Device, Stream};

fn main() {
    // Fetch some generic device information
    let devices = Device::enumerate();
    println!("Found {} devices.", devices.len());

    println!("Choosing first device (index = 0)");
    let dev = Device::new(devices[0]).expect("Failed to open device");

    // Fetch name and ID
    println!("[{}] {}", dev.index, dev.name);

    println!("Found {} formats.", dev.formats().len());

    // Create the stream
    let mut stream = Stream::new(&dev, &dev.formats()[0]).expect("Failed to create stream");

    println!(
        "[0] {} ({}x{}@{})",
        stream.format().fourcc,
        stream.format().width,
        stream.format().height,
        stream.format().fps
    );

    // Capture some frames
    let mut rgb_buffer = Vec::new();

    // Warmup
    stream.advance();

    let mut window = Window::new(
        "Test - ESC to exit",
        stream.format().width as usize,
        stream.format().height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let size: usize = (stream.format().width * stream.format().height) as usize;

    loop {
        if window.is_key_down(Key::Escape) || !window.is_open() {
            break;
        }
        if !stream.poll() {
            continue;
        }

        stream.advance();
        stream
            .read(&mut rgb_buffer)
            .expect("Failed to capture frame");

        let mut buffer = vec![Default::default(); size];
        let mut index: usize = 0;
        for (i, _) in (rgb_buffer).iter().step_by(3).enumerate() {
            let r = rgb_buffer[i];
            let g = rgb_buffer[i + 1];
            let b = rgb_buffer[i + 2];

            println!("{}, {}, {}", r, g, b);

            let rgb = from_u8_rgb(g,b,r);
            buffer[index] = rgb;
            index += 1;
        }

        window
            .update_with_buffer(
                &buffer,
                stream.format().width as usize,
                stream.format().height as usize
            )
            .expect("Failed to update buffer");
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (b << 16) | (g << 8) | r
}
