use crate::strip::EffectIterator;
use palette::{Mix, Srgb};

/// Energy bar effect
///
/// This effect displays a bar that moves up and down the strip, similar to an audio
/// level meter or energy indicator. The bar can optionally have a gradient.
///
/// # Arguments
///
/// * `count` - The number of pixels in the strip.
/// * `start_colour` - The colour at the bottom of the bar.
/// * `end_colour` - The colour at the top of the bar.
/// * `gradient` - Whether to use a gradient from start to end colour.
/// * `speed` - The speed at which the bar moves (pixels per tick).
///
/// # Examples
///
/// ```
/// use smart_led_effects::strip::EnergyBar;
/// use palette::Srgb;
///
/// let count = 10;
/// let start_colour = Some(Srgb::new(0.0, 1.0, 0.0));
/// let end_colour = Some(Srgb::new(1.0, 0.0, 0.0));
/// let gradient = Some(true);
/// let speed = Some(0.5);
///
/// let mut effect = EnergyBar::new(count, start_colour, end_colour, gradient, speed);
/// ```
pub struct EnergyBar {
    count: usize,
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    speed: f32,
    current_height: f32,
    direction: f32,
}

impl EnergyBar {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 1.0, 0.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    const DEFAULT_SPEED: f32 = 0.5;

    pub fn new(
        count: usize,
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
        speed: Option<f32>,
    ) -> Self {
        EnergyBar {
            count,
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(true),
            speed: speed.unwrap_or(Self::DEFAULT_SPEED),
            current_height: 0.0,
            direction: 1.0,
        }
    }

    pub fn set_height(&mut self, height: f32) {
        self.current_height = height.clamp(0.0, self.count as f32);
    }
}

impl EffectIterator for EnergyBar {
    fn name(&self) -> &'static str {
        "EnergyBar"
    }

    fn next(&mut self) -> Option<Vec<Srgb<u8>>> {
        // Update height
        self.current_height += self.direction * self.speed;

        // Bounce at edges
        if self.current_height >= self.count as f32 {
            self.current_height = self.count as f32;
            self.direction = -1.0;
        } else if self.current_height <= 0.0 {
            self.current_height = 0.0;
            self.direction = 1.0;
        }

        let pixels = self.current_height as usize;
        let mut out = vec![Srgb::new(0.0, 0.0, 0.0).into_format(); self.count];

        if self.gradient {
            for (i, pixel) in out.iter_mut().take(pixels).enumerate() {
                *pixel = self
                    .start_colour
                    .mix(self.end_colour, i as f32 / self.count as f32)
                    .into_format();
            }
        } else {
            let colour = self
                .start_colour
                .mix(self.end_colour, self.current_height / self.count as f32);
            for pixel in out.iter_mut().take(pixels) {
                *pixel = colour.into_format();
            }
        }

        Some(out)
    }
}
