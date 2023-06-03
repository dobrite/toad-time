use eg_pcf::{include_pcf, text::PcfTextStyle, PcfFont};
use embedded_graphics::{
    geometry::Dimensions, pixelcolor::BinaryColor, prelude::Point, primitives::Rectangle,
    text::Text,
};

type Font = PcfTextStyle<'static, BinaryColor>;

const SMOL_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-7.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');
const BIGGE_FONT: PcfFont = include_pcf!("src/assets/fonts/FrogPrincess-10.pcf", 'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '/' | '%');

pub struct Fonts {
    bigge_font: Font,
    smol_font: Font,
}

impl Fonts {
    pub fn new() -> Self {
        let bigge_font = PcfTextStyle::new(&BIGGE_FONT, BinaryColor::On);
        let smol_font = PcfTextStyle::new(&SMOL_FONT, BinaryColor::On);

        Self {
            bigge_font,
            smol_font,
        }
    }

    pub fn bigge_font_bounding_box(&self, str: &str, point: Point) -> Rectangle {
        let text = Text::new(str, point, self.bigge_font);
        let font_bb = BIGGE_FONT.bounding_box;
        let mut bb = text.bounding_box();
        bb.top_left.y += font_bb.top_left.y + 2;
        bb
    }

    pub fn smol_font_bounding_box(&self, str: &str, point: Point) -> Rectangle {
        let text = Text::new(str, point, self.smol_font);
        let font_bb = SMOL_FONT.bounding_box;
        let mut bb = text.bounding_box();
        bb.top_left.y += font_bb.top_left.y;
        bb.size.height += 2;
        bb
    }

    pub fn bigge_text<'a>(&'a self, str: &'a str, point: Point) -> Text<PcfTextStyle<BinaryColor>> {
        Text::new(str, point, self.bigge_font)
    }

    pub fn smol_text<'a>(&'a self, str: &'a str, point: Point) -> Text<PcfTextStyle<BinaryColor>> {
        Text::new(str, point, self.smol_font)
    }
}
