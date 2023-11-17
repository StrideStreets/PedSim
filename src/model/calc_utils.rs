use krabmaga::engine::location::Real2D;

pub fn normalize_motion_vector(loc: Real2D, dest: Real2D) -> (f32, f32) {
    let initial_vector_magnitude: f32 =
        ((dest.x - loc.x).powf(2.0) + (dest.y - loc.y).powf(2.0)).sqrt();
    let dir_x: f32 = (dest.x - loc.x) / initial_vector_magnitude;
    let dir_y: f32 = (dest.y - loc.y) / initial_vector_magnitude;
    (dir_x, dir_y)
}
