use crate::console_log;
use candle_core::{Device, Result, Tensor};
use candle_transformers::models::mixformer::Config as MixConfig;
use candle_transformers::models::quantized_mistral::{Config as MistralConfig, Model as QMistral};
use candle_transformers::models::quantized_mixformer::MixFormerSequentialForCausalLM;
use serde::{Deserialize, Serialize};

use tokenizers::Tokenizer;

#[derive(Serialize, Deserialize)]
pub struct ModelData {
    pub tokenizer: Vec<u8>,
    pub model: Vec<u8>,
    pub config: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]

pub struct ModelName {
    pub _name_or_path: String,
}

pub enum Model {
    QMixFormer(QMixFormer),
    QMistral(QMistralModel),
}

pub struct QMixFormer {
    pub config: MixConfig,
    pub model_instance: MixFormerSequentialForCausalLM,
    pub tokenizer: Tokenizer,
}

pub struct QMistralModel {
    pub config: MistralConfig,
    pub model_instance: QMistral,
    pub tokenizer: Tokenizer,
}

impl Model {
    pub fn forward(&mut self, input: &Tensor, token_len: usize) -> Result<Tensor> {
        match self {
            Model::QMixFormer(model) => model.model_instance.forward(input),
            Model::QMistral(model) => model.model_instance.forward(input, token_len),
        }
    }

    pub fn tokenizer(&self) -> &Tokenizer {
        match self {
            Model::QMixFormer(model) => &model.tokenizer,
            Model::QMistral(model) => &model.tokenizer,
        }
    }

    pub fn load(md: ModelData, model_type: &str) -> Result<Self> {
        console_log!("loading model");
        let device = Device::Cpu;
        let tokenizer = Tokenizer::from_bytes(&md.tokenizer)
            .map_err(|m| candle_core::Error::Msg(m.to_string()))?;

        match model_type {
            "QMixFormer" => {
                let name: ModelName = serde_json::from_slice(&md.config).unwrap();
                let config: MixConfig = serde_json::from_slice(&md.config).unwrap();
                console_log!("config loaded {:?}", name);
                console_log!("weights len: {:?}", md.model.len());
                let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf_buffer(
                    &md.model, &device,
                )?;
                console_log!("weights loaded");
                let model_instance = if name._name_or_path == "microsoft/phi-2" {
                    MixFormerSequentialForCausalLM::new_v2(&config, vb)?
                } else {
                    MixFormerSequentialForCausalLM::new(&config, vb)?
                };
                Ok(Model::QMixFormer(QMixFormer {
                    config,
                    model_instance,
                    tokenizer,
                }))
            }
            "QMistral" => {
                let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf_buffer(
                    &md.model, &device,
                )?;
                let config: MistralConfig = serde_json::from_slice(&md.config).unwrap();
                let model_instance = QMistral::new(&config, vb)?;
                Ok(Model::QMistral(QMistralModel {
                    config,
                    model_instance,
                    tokenizer,
                }))
            }
            _ => Err(candle_core::Error::Msg("Unknown model type".into())),
        }
    }
}
