use crate::color::Rgb;
use crate::framebuffer::Framebuffer;

// Triángulo relleno
pub fn triangle(fb: &mut Framebuffer, a: [f32; 3], b: [f32; 3], c: [f32; 3], color: Rgb) {
    let min_x = a[0].min(b[0]).min(c[0]).floor().max(0.0) as i32;
    let max_x = a[0].max(b[0]).max(c[0]).ceil().min(fb.width as f32 - 1.0) as i32;
    let min_y = a[1].min(b[1]).min(c[1]).floor().max(0.0) as i32;
    let max_y = a[1].max(b[1]).max(c[1]).ceil().min(fb.height as f32 - 1.0) as i32;

    let denom = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
    if denom.abs() < 1e-6 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let w0 = ((b[1] - c[1]) * (px - c[0]) + (c[0] - b[0]) * (py - c[1])) / denom;
            let w1 = ((c[1] - a[1]) * (px - c[0]) + (a[0] - c[0]) * (py - c[1])) / denom;
            let w2 = 1.0 - w0 - w1;
            // Dentro si los tres pesos >= 0
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let z = w0 * a[2] + w1 * b[2] + w2 * c[2];
                fb.point(x, y, z, color);
            }
        }
    }
}
