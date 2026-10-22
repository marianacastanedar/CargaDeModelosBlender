mod color;
mod framebuffer;
mod obj;
mod triangle;

use color::Rgb;
use framebuffer::Framebuffer;
use obj::Model;
use triangle::triangle;

const WIDTH: usize = 1000;
const HEIGHT: usize = 800;

const COLOR_ARRIBA: Rgb = Rgb::new(218, 236, 255); // celeste
const COLOR_ABAJO: Rgb = Rgb::new(36, 34, 133); // azul marino
const ROTACION_Y: f32 = 35.0; // giro grados
const INCLINACION_X: f32 = 25.0; // inclinación
const TAMANO: f32 = 0.85; // % pantalla

// Gira, centra y escala
fn prepare_model(model: &Model) -> (Vec<[f32; 3]>, f32) {
    let ay = ROTACION_Y.to_radians();
    let ax = INCLINACION_X.to_radians();

    // Centro
    let mut mn = [f32::MAX; 3];
    let mut mx = [f32::MIN; 3];
    for v in &model.vertices {
        for k in 0..3 {
            mn[k] = mn[k].min(v[k]);
            mx[k] = mx[k].max(v[k]);
        }
    }
    let center = [(mn[0] + mx[0]) / 2.0, (mn[1] + mx[1]) / 2.0, (mn[2] + mx[2]) / 2.0];
    let mid_height = center[1];

    // Girar en Y y luego  X
    let rotated: Vec<[f32; 3]> = model
        .vertices
        .iter()
        .map(|v| {
            let (x, y, z) = (v[0] - center[0], v[1] - center[1], v[2] - center[2]);
            let (x1, y1, z1) = (ay.cos() * x + ay.sin() * z, y, -ay.sin() * x + ay.cos() * z);
            [x1, ax.cos() * y1 - ax.sin() * z1, ax.sin() * y1 + ax.cos() * z1]
        })
        .collect();

    // Tamaño ya girado
    let mut rmn = [f32::MAX; 3];
    let mut rmx = [f32::MIN; 3];
    for p in &rotated {
        for k in 0..3 {
            rmn[k] = rmn[k].min(p[k]);
            rmx[k] = rmx[k].max(p[k]);
        }
    }
    let scale = TAMANO * (WIDTH as f32 / (rmx[0] - rmn[0])).min(HEIGHT as f32 / (rmx[1] - rmn[1]));
    let rc = [(rmn[0] + rmx[0]) / 2.0, (rmn[1] + rmx[1]) / 2.0];

    // A pantalla (Y voltwa)
    let screen = rotated
        .iter()
        .map(|p| {
            [
                WIDTH as f32 / 2.0 + (p[0] - rc[0]) * scale,
                HEIGHT as f32 / 2.0 - (p[1] - rc[1]) * scale,
                p[2],
            ]
        })
        .collect();
    (screen, mid_height)
}

// Dibuja todos los triángulos
fn render(fb: &mut Framebuffer, model: &Model, screen: &[[f32; 3]], mid_height: f32) {
    // Caras de 3 en 3 índices
    for t in model.indices.chunks_exact(3) {
        let (i0, i1, i2) = (t[0], t[1], t[2]);

        // Color según altura del triángulo
        let h = (model.vertices[i0][1] + model.vertices[i1][1] + model.vertices[i2][1]) / 3.0;
        let color = if h >= mid_height { COLOR_ARRIBA } else { COLOR_ABAJO };

        triangle(fb, screen[i0], screen[i1], screen[i2], color);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let solo_guardar = args.iter().any(|a| a == "--guardar");
    let path = args.iter().find(|a| !a.starts_with("--")).map(|s| s.as_str()).unwrap_or("assets/naveterodactilo.obj");

    let model = match obj::load_obj(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    println!("Vértices: {}, triángulos: {}", model.vertices.len(), model.indices.len() / 3);

    let (screen, mid_height) = prepare_model(&model);
    let mut fb = Framebuffer::new(WIDTH, HEIGHT, Rgb::new(20, 20, 20));
    fb.clear();
    render(&mut fb, &model, &screen, mid_height);

    #[cfg(feature = "ventana")]
    {
        if !solo_guardar {
            mostrar_ventana(&fb);
            return;
        }
    }

    // Solo guarda la imagen
    let _ = solo_guardar;
    match fb.save_png("render.png") {
        Ok(_) => println!("Imagen guardada: render.png"),
        Err(e) => eprintln!("No se pudo guardar: {}", e),
    }
}

#[cfg(feature = "ventana")]
fn mostrar_ventana(fb: &Framebuffer) {
    use raylib::prelude::*;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Software Renderer")
        .build();
    rl.set_target_fps(60);

    // Textura con la imagen renderizada
    let image = Image::load_image_from_mem(".png", &fb.to_png_bytes()).expect("imagen");
    let texture = rl.load_texture_from_image(&thread, &image).expect("textura");

    while !rl.window_should_close() {
        // Tecla S guarda la imagen
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            match fb.save_png("render.png") {
                Ok(_) => println!("Imagen guardada: render.png"),
                Err(e) => eprintln!("No se pudo guardar: {}", e),
            }
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
