use crate::strip::EffectIterator;
use palette::{FromColor, Hsv, Srgb};

/// Brightness filter effect
///
/// This effect wraps another effect and applies a brightness multiplier to all pixels.
///
/// # Arguments
///
/// * `effect` - The effect to wrap.
/// * `brightness` - The brightness multiplier (0.0 to 1.0).
///
/// # Examples
///
/// ```
/// use smart_led_effects::strip::{Brightness, Rainbow, EffectIterator};
///
/// let count = 10;
/// let effect = Box::new(Rainbow::new(count, None));
///
/// let mut brightness_effect = Brightness::new(effect, 0.5);
/// ```
pub struct Brightness {
    effect: Box<dyn EffectIterator>,
    brightness: f32,
}

impl Brightness {
    pub fn new(effect: Box<dyn EffectIterator>, brightness: f32) -> Self {
        Brightness {
            effect,
            brightness: brightness.clamp(0.0, 1.0),
        }
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.0, 1.0);
    }
}

impl EffectIterator for Brightness {
    fn name(&self) -> &'static str {
        "Brightness"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.effect.next().map(|frame| {
            frame
                .iter()
                .map(|pixel| {
                    let mut hsv = Hsv::from_color(pixel.into_format::<f32>());
                    hsv.value *= self.brightness;
                    Srgb::from_color(hsv).into_format::<u8>()
                })
                .collect()
        })
    }
}
