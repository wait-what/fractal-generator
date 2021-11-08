use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

fn main() {
    let iterations: u8 = std::env::args().nth(1).expect("Provide amount of iterations as an argument (0-10 or so)").parse().unwrap();
    if iterations > 8 {
        println!("Warning: above 8 iterations - very resource intensive.");
    };

    let mut image = DynamicImage::new_rgba8(1, 1);
    image.put_pixel(0, 0, Rgba([0x2c, 0x07, 0x35, 0xff]));

    for iteration in 0..iterations {
        let (width, height) = image.dimensions();
        let mut new_image = DynamicImage::new_rgba8(width * 3, height * 3);

        for x in 0..3 {
            for y in 0..3 {
                for part_x in 0..width {
                    for part_y in 0..height {
                        new_image.put_pixel(
                            x * width + part_x,
                            y * height + part_y,
                            if x == 1 && y == 1 {
                                Rgba(match iteration % 4 {
                                    0 => [0xF5, 0xB5, 0xFC, 0xFF],
                                    1 => [0x96, 0xF7, 0xD2, 0xFF],
                                    2 => [0xF0, 0xF6, 0x96, 0xFF],
                                    3 => [0xFC, 0xB1, 0xB1, 0xFF],
                                    _ => unreachable!()
                                })
                            } else {
                                image.get_pixel(part_x, part_y)
                            },
                        );
                    }
                }
            }
        }

        image = new_image;
    }

    image.save("output.png").unwrap();
}
