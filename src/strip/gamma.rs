use crate::strip::EffectIterator;
use palette::Srgb;

/// Gamma correction effect
///
/// This effect wraps another effect and applies gamma correction to all pixels.
/// Gamma correction helps compensate for the non-linear brightness perception of LEDs.
///
/// # Arguments
///
/// * `effect` - The effect to wrap.
/// * `gamma` - The gamma value (typically 2.2 for LEDs, range 0.1 to 5.0).
///
/// # Examples
///
/// ```
/// use smart_led_effects::strip::{Gamma, Rainbow, EffectIterator};
///
/// let count = 10;
/// let effect = Box::new(Rainbow::new(count, None));
///
/// let mut gamma_effect = Gamma::new(effect, 2.2);
/// ```
pub struct Gamma {
    effect: Box<dyn EffectIterator>,
    gamma: f32,
    lookup_table: [u8; 256],
}

impl Gamma {
    pub fn new(effect: Box<dyn EffectIterator>, gamma: f32) -> Self {
        let gamma = gamma.clamp(0.1, 5.0);
        let lookup_table = Self::build_lookup_table(gamma);
        Gamma {
            effect,
            gamma,
            lookup_table,
        }
    }

    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma.clamp(0.1, 5.0);
        self.lookup_table = Self::build_lookup_table(self.gamma);
    }

    fn build_lookup_table(gamma: f32) -> [u8; 256] {
        let mut table = [0u8; 256];
        for (i, value) in table.iter_mut().enumerate() {
            let normalized = i as f32 / 255.0;
            let corrected = normalized.powf(gamma);
            *value = (corrected * 255.0).round() as u8;
        }
        table
    }

    fn apply_gamma(&self, color: Srgb<u8>) -> Srgb<u8> {
        Srgb::new(
            self.lookup_table[color.red as usize],
            self.lookup_table[color.green as usize],
            self.lookup_table[color.blue as usize],
        )
    }
}

impl EffectIterator for Gamma {
    fn name(&self) -> &'static str {
        "Gamma"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        self.effect.next().map(|frame| {
            frame
                .iter()
                .map(|pixel| self.apply_gamma(*pixel))
                .collect()
        })
    }
}
