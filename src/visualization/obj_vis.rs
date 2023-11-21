use crate::model::state::state::ModelState;
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::{bevy::prelude::Image, visualization::fields::number_grid_2d::BatchRender};

impl BatchRender<ModelState> for SparseNumberGrid2D<u8> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_value(loc) {
            Some(val) => [0u8, 0u8, 0u8, 255u8],
            None => [255u8, 255u8, 255u8, 255u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.obj_grid.texture()
    }
}
