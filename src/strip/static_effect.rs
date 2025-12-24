use crate::strip::EffectIterator;
use palette::Srgb;

/// Static effect
///
/// This effect renders a constant frame that never changes.
///
/// # Arguments
///
/// * `frame` - The frame to render constantly.
///
/// # Examples
///
/// ```
/// use smart_led_effects::strip::Static;
/// use palette::Srgb;
///
/// let count = 10;
/// let frame = vec![Srgb::new(255, 0, 0); count];
///
/// let mut effect = Static::new(frame);
/// ```
#[derive(Debug, Clone)]
pub struct Static {
    frame: Vec<Srgb<u8>>,
}

impl Static {
    pub fn new(frame: Vec<Srgb<u8>>) -> Self {
        Static { frame }
    }
}

impl EffectIterator for Static {
    fn name(&self) -> &'static str {
        "Static"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        Some(self.frame.clone())
    }
}
