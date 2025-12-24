use crate::strip::EffectIterator;
use palette::Srgb;

/// Concat effect
///
/// This effect wraps around N effects and runs them in order.
/// When an effect returns `None` (indicating it has ended), the concat effect
/// moves on to the next effect in the sequence. When all effects have ended,
/// this effect returns `None`.
///
/// # Arguments
///
/// * `effects` - A vector of boxed effects to run in sequence.
///
/// # Examples
///
/// ```
/// use smart_led_effects::strip::{Concat, Rainbow, Wipe, EffectIterator};
///
/// let count = 10;
/// let effects: Vec<Box<dyn EffectIterator>> = vec![
///     Box::new(Rainbow::new(count, None)),
///     Box::new(Wipe::colour_wipe(count, None, false)),
/// ];
///
/// let mut effect = Concat::new(effects);
/// ```
pub struct Concat {
    effects: Vec<Box<dyn EffectIterator>>,
    current_index: usize,
}

impl Concat {
    pub fn new(effects: Vec<Box<dyn EffectIterator>>) -> Self {
        Concat {
            effects,
            current_index: 0,
        }
    }
}

impl EffectIterator for Concat {
    fn name(&self) -> &'static str {
        "Concat"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        while self.current_index < self.effects.len() {
            if let Some(frame) = self.effects[self.current_index].next() {
                return Some(frame);
            }
            // Current effect ended, move to next
            self.current_index += 1;
        }
        // All effects have ended
        None
    }
}
