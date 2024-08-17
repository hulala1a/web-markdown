pub enum Model {
    QuantizedT5(QuantizedT5),
}

pub struct QuantizedT5 {
    pub config: Config,
    pub quantized_model: T5ForConditionalGeneration,
    pub tokenizer: TokenizerA,
}
