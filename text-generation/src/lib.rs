mod utils;
mod worker;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// use crate::diff::update;

use crate::worker::{Model as M, ModelData};
use candle_core::{DType, Device, Tensor};
use candle_transformers::generation::LogitsProcessor;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate console_error_panic_hook;
use std::panic;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Model {
    model: M,
    logits_processor: LogitsProcessor,
    tokens: Vec<u32>,
    repeat_penalty: f32,
}

#[wasm_bindgen]
impl Model {
    fn process(&mut self, tokens: &[u32]) -> candle_core::Result<String> {
        set_panic_hook();
        const REPEAT_LAST_N: usize = 64;
        let dev = Device::Cpu;
        let input = Tensor::new(tokens, &dev)?.unsqueeze(0)?;
        let logits = self.model.forward(&input, tokens.len())?;
        let logits = logits.squeeze(0)?.to_dtype(DType::F32)?;
        let logits = if self.repeat_penalty == 1. || tokens.is_empty() {
            logits
        } else {
            let start_at = self.tokens.len().saturating_sub(REPEAT_LAST_N);
            candle_transformers::utils::apply_repeat_penalty(
                &logits,
                self.repeat_penalty,
                &self.tokens[start_at..],
            )?
        };

        let next_token = self.logits_processor.sample(&logits)?;
        self.tokens.push(next_token);
        let text = match self.model.tokenizer().decode(&[next_token], false) {
            Ok(token) => token,
            Err(e) => {
                console_log!("error decoding token: {:?}", e);
                "".to_string()
            }
        };
        Ok(text)
    }
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(weights: Vec<u8>, tokenizer: Vec<u8>, config: Vec<u8>) -> Result<Model, JsError> {
        set_panic_hook();
        console_log!("loading");
        let model = M::load(
            ModelData {
                tokenizer,
                config,
                model: weights,
            },
            "QMixFormer",
        );
        let logits_processor = LogitsProcessor::new(299792458, None, None);
        match model {
            Ok(model) => Ok(Self {
                model,
                logits_processor,
                tokens: vec![],
                repeat_penalty: 1.,
            }),
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    #[wasm_bindgen]
    pub fn init_with_prompt(
        &mut self,
        prompt: String,
        temp: f64,
        top_p: f64,
        repeat_penalty: f32,
        seed: u64,
    ) -> Result<String, JsError> {
        set_panic_hook();
        match &mut self.model {
            M::QMixFormer(m) => m.model_instance.clear_kv_cache(),
            M::QMistral(m) => todo!(),
        }
        let temp = if temp <= 0. { None } else { Some(temp) };
        let top_p = if top_p <= 0. || top_p >= 1. {
            None
        } else {
            Some(top_p)
        };
        self.logits_processor = LogitsProcessor::new(seed, temp, top_p);
        self.repeat_penalty = repeat_penalty;
        self.tokens.clear();
        let tokens = self
            .model
            .tokenizer()
            .encode(prompt, true)
            .map_err(|m| JsError::new(&m.to_string()))?
            .get_ids()
            .to_vec();
        let text = self
            .process(&tokens)
            .map_err(|m| JsError::new(&m.to_string()))?;
        Ok(text)
    }

    #[wasm_bindgen]
    pub fn next_token(&mut self) -> Result<String, JsError> {
        set_panic_hook();
        let last_token = *self.tokens.last().unwrap();
        let text = self
            .process(&[last_token])
            .map_err(|m| JsError::new(&m.to_string()))?;
        Ok(text)
    }
}
