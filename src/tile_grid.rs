use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};

pub struct TileGrid {
    tiles: Size,
    tile_size: Size,
}

impl TileGrid {
    pub fn new(tiles: Size, tile_size: Size) -> Self {
        Self { tiles, tile_size }
    }

    pub fn get_rect(&self, index: usize) -> Rectangle {
        let tile_x = index % self.tiles.width as usize;
        let tile_y = index / self.tiles.width as usize;
        let x = tile_x * self.tile_size.width as usize;
        let y = tile_y * self.tile_size.height as usize;

        let point = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
        Rectangle::new(point, self.tile_size)
    }
}
