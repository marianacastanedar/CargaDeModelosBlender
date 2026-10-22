use crate::color::Rgb;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGBA, 4 bytes por pixel
    zbuffer: Vec<f32>,   // profundidad por pixel
    pub background: Rgb,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background: Rgb) -> Self {
        let mut fb = Framebuffer {
            width,
            height,
            pixels: vec![0; width * height * 4],
            zbuffer: vec![f32::MIN; width * height],
            background,
        };
        fb.clear();
        fb
    }

    // Limpia pantalla y profundidad
    pub fn clear(&mut self) {
        let bg = self.background;
        for p in self.pixels.chunks_exact_mut(4) {
            p.copy_from_slice(&[bg.r, bg.g, bg.b, 255]);
        }
        self.zbuffer.fill(f32::MIN);
    }

    // Pinta un pixel si está más cerca
    pub fn point(&mut self, x: i32, y: i32, z: f32, color: Rgb) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let i = y as usize * self.width + x as usize;
        if z > self.zbuffer[i] {
            self.zbuffer[i] = z;
            self.pixels[i * 4..i * 4 + 4].copy_from_slice(&[color.r, color.g, color.b, 255]);
        }
    }

    // Guarda la imagen como PNG
    pub fn save_png(&self, path: &str) -> Result<(), image::ImageError> {
        image::save_buffer(
            path,
            &self.pixels,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgba8,
        )
    }

    // PNG en memoria (para la ventana)
    #[allow(dead_code)]
    pub fn to_png_bytes(&self) -> Vec<u8> {
        let mut bytes = std::io::Cursor::new(Vec::new());
        let img = image::RgbaImage::from_raw(self.width as u32, self.height as u32, self.pixels.clone()).unwrap();
        img.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
        bytes.into_inner()
    }
}
