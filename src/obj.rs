use std::fs;

pub struct Model {
    pub vertices: Vec<[f32; 3]>, // líneas y
    pub indices: Vec<usize>,     // un triángulo
}

// Lee.obj 
pub fn load_obj(path: &str) -> Result<Model, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("No se pudo abrir {}: {}", path, e))?;
    let mut model = Model { vertices: Vec::new(), indices: Vec::new() };

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let v: Vec<f32> = parts.take(3).filter_map(|n| n.parse().ok()).collect();
                if v.len() == 3 {
                    model.vertices.push([v[0], v[1], v[2]]);
                }
            }
            Some("f") => {
                let mut face: Vec<usize> = Vec::new();
                for token in parts {
                    let n: i64 = token.split('/').next().unwrap().parse().map_err(|_| format!("Cara inválida: {}", line))?;
                    let idx = if n < 0 { model.vertices.len() as i64 + n } else { n - 1 };
                    face.push(idx as usize);
                }
                // Cuadrados se parten en triángulos
                for i in 1..face.len().saturating_sub(1) {
                    model.indices.extend_from_slice(&[face[0], face[i], face[i + 1]]);
                }
            }
            _ => {}
        }
    }
    Ok(model)
}
