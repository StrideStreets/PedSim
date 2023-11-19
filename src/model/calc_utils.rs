use krabmaga::engine::location::Int2D;

pub fn normalize_motion_vector(loc: Int2D, dest: Int2D) -> (f32, f32) {
    let dx = dest.x as f32;
    let dy = dest.y as f32;
    let lx = loc.x as f32;
    let ly = loc.x as f32;
    let initial_vector_magnitude: f32 = ((dx - lx).powf(2.0) + (dy - ly).powf(2.0)).sqrt();
    let dir_x = ((dx - lx) / initial_vector_magnitude);
    let dir_y = ((dy - ly) / initial_vector_magnitude);
    (dir_x, dir_y)
}
