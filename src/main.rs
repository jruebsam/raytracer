pub mod vec3;

use vec3::Vec3;
use zzz::ProgressBar;

fn main() {
    let width = 512;
    let height = 512;

    let mut pb = ProgressBar::with_target(width * height);

    let mut img = image::ImageBuffer::new(512, 512);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        pb.add(1);
        let dx = x as f64 / width as f64;
        let dy = y as f64 / height as f64;

        let r = (4. * dx).sin() * (3. * dy).cos();
        let g = (2. * dx).cos() * dy.sin();
        let b = (32. * dx).sin() * (2. * dy).cos();

        let r = (255. * r).clamp(0.0, 255.0) as u8;
        let g = (255. * g).clamp(0.0, 255.0) as u8;
        let b = (255. * b).clamp(0.0, 255.0) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    img.save_with_format("test.png", image::ImageFormat::Png)
        .unwrap();
}
