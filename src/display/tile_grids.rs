use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};

struct TileGrid {
    tiles: Size,
    tile_size: Size,
}

impl TileGrid {
    fn new(tiles: Size, tile_size: Size) -> Self {
        Self { tiles, tile_size }
    }

    fn get_rect(&self, index: usize) -> Rectangle {
        let tile_x = index % self.tiles.width as usize;
        let tile_y = index / self.tiles.width as usize;
        let x = tile_x * self.tile_size.width as usize;
        let y = tile_y * self.tile_size.height as usize;

        let point = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
        Rectangle::new(point, self.tile_size)
    }
}

pub struct TileGrids {
    frogge: TileGrid,
    play_pause: TileGrid,
    pwm: TileGrid,
}

impl TileGrids {
    pub fn new() -> Self {
        let frogge = TileGrid::new(Size::new(4, 2), Size::new(22, 22));
        let play_pause = TileGrid::new(Size::new(2, 1), Size::new(16, 16));
        let pwm = TileGrid::new(Size::new(5, 2), Size::new(26, 16));

        Self {
            frogge,
            play_pause,
            pwm,
        }
    }

    pub fn get_frogge_rect(&self, index: usize) -> Rectangle {
        self.frogge.get_rect(index)
    }

    pub fn get_play_pause_rect(&self, index: usize) -> Rectangle {
        self.play_pause.get_rect(index)
    }

    pub fn get_pwm_rect(&self, index: usize) -> Rectangle {
        self.pwm.get_rect(index)
    }
}
