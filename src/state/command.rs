use defmt::Format;

#[derive(Clone, Copy, Format)]
pub enum Command {
    EncoderRight,
    EncoderLeft,
    EncoderPress,
    PagePress,
    PlayPress,
    BpmPress,
}
