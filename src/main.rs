const ITERATIONS: usize = 6;

use image::{GenericImage, GenericImageView, DynamicImage, Rgba};

fn main() {
    let mut image = DynamicImage::new_rgba8(3, 3);

    for x in 0..3 {
        for y in 0..3 {
            image.put_pixel(x, y, Rgba(
                if x == 1 && y == 1 { [0xff, 0xff, 0xff, 0xff] } else { [0x00, 0x00, 0x00, 0xff] }
            ));
        }
    }

    for _ in 0..ITERATIONS {
        let (width, height) = image.dimensions();
        let mut new_image = DynamicImage::new_rgba8(width * 3, height * 3);

        for x in 0..3 {
            for y in 0..3 {
                if x == 1 && y == 1 {
                    for part_x in 0..width {
                        for part_y in 0..height {
                            new_image.put_pixel(x * width + part_x, y * height + part_y, Rgba([0xff, 0xff, 0xff, 0xff]));
                        }
                    }
                } else {
                    for part_x in 0..width {
                        for part_y in 0..height {
                            new_image.put_pixel(x * width + part_x, y * height + part_y, image.get_pixel(part_x, part_y));
                        }
                    }
                }
            }
        }

        image = new_image;
    }

    image.save("output.png").unwrap();
}
