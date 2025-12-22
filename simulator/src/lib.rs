use smart_led_effects::{strip, Srgb};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LedSimulator {
    effect: Box<dyn strip::EffectIterator>,
    led_count: usize,
}

#[wasm_bindgen]
impl LedSimulator {
    #[wasm_bindgen(constructor)]
    pub fn new(led_count: usize, effect_name: &str) -> Result<LedSimulator, JsValue> {
        let effect = strip::get_default_effect(led_count, effect_name)
            .ok_or_else(|| JsValue::from_str("Unknown effect"))?;

        Ok(LedSimulator { effect, led_count })
    }

    #[wasm_bindgen(js_name = setEffect)]
    pub fn set_effect(&mut self, effect_name: &str) -> Result<(), JsValue> {
        let effect = strip::get_default_effect(self.led_count, effect_name)
            .ok_or_else(|| JsValue::from_str("Unknown effect"))?;
        self.effect = effect;
        Ok(())
    }

    #[wasm_bindgen(js_name = nextFrame)]
    pub fn next_frame(&mut self) -> Vec<u8> {
        if let Some(colors) = self.effect.next() {
            colors_to_bytes(&colors)
        } else {
            vec![0; self.led_count * 3]
        }
    }

    #[wasm_bindgen(js_name = getEffectName)]
    pub fn get_effect_name(&self) -> String {
        self.effect.name().to_string()
    }
}

#[wasm_bindgen(js_name = listEffects)]
pub fn list_effects() -> Vec<String> {
    strip::list()
}

fn colors_to_bytes(colors: &[Srgb<u8>]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(colors.len() * 3);
    for color in colors {
        bytes.push(color.red);
        bytes.push(color.green);
        bytes.push(color.blue);
    }
    bytes
}
