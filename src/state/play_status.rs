use defmt::Format;

#[derive(Clone, Copy, Format)]
pub enum PlayStatus {
    Playing,
    Paused,
}
