use embedded_graphics::pixelcolor::BinaryColor;
use tinybmp::Bmp as TinyBmp;

pub type Bmp = TinyBmp<'static, BinaryColor>;

const CARET: &[u8; 78] = include_bytes!("../assets/icons/caret.bmp");
const CLOCK: &[u8; 1318] = include_bytes!("../assets/icons/clock.bmp");
const DICE: &[u8; 1334] = include_bytes!("../assets/icons/die.bmp");
const FROGGE: &[u8; 4950] = include_bytes!("../assets/icons/spin-sprite-sheet.bmp"); // 88x44
const PLAY_PAUSE: &[u8; 1590] = include_bytes!("../assets/icons/play-pause.bmp");
const POINTER_LEFT: &[u8; 630] = include_bytes!("../assets/icons/pointer-left.bmp");
const POINTER_RIGHT: &[u8; 630] = include_bytes!("../assets/icons/pointer-right.bmp");
const PWM: &[u8; 12_598] = include_bytes!("../assets/icons/pwm-sprite-sheet.bmp");
const STEP_OFF: &[u8; 134] = include_bytes!("../assets/icons/step-off.bmp");
const STEP_ON: &[u8; 134] = include_bytes!("../assets/icons/step-on.bmp");

pub struct Bmps {
    pub caret: Bmp,
    pub clock: Bmp,
    pub dice: Bmp,
    pub frogge: Bmp,
    pub play_pause: Bmp,
    pub pointer_left: Bmp,
    pub pointer_right: Bmp,
    pub pwm: Bmp,
    pub step_off: Bmp,
    pub step_on: Bmp,
}

impl Bmps {
    pub fn new() -> Self {
        let caret = TinyBmp::from_slice(CARET).unwrap();
        let clock = TinyBmp::from_slice(CLOCK).unwrap();
        let dice = TinyBmp::from_slice(DICE).unwrap();
        let frogge = TinyBmp::from_slice(FROGGE).unwrap();
        let play_pause = TinyBmp::from_slice(PLAY_PAUSE).unwrap();
        let pointer_left = TinyBmp::from_slice(POINTER_LEFT).unwrap();
        let pointer_right = TinyBmp::from_slice(POINTER_RIGHT).unwrap();
        let pwm = TinyBmp::from_slice(PWM).unwrap();
        let step_off = TinyBmp::from_slice(STEP_OFF).unwrap();
        let step_on = TinyBmp::from_slice(STEP_ON).unwrap();

        Self {
            caret,
            clock,
            dice,
            frogge,
            play_pause,
            pointer_left,
            pointer_right,
            pwm,
            step_off,
            step_on,
        }
    }
}
