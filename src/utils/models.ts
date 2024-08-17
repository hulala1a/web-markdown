interface ModelConfig {
  base_url: string;
  model: string | string[];
  tokenizer: string;
  config: string;
  quantized: boolean;
  seq_len: number;
  size: string;
}

export const MODELS: Map<string, ModelConfig> = new Map([
  [
    'phi_1_5_q4k',
    {
      base_url: 'https://huggingface.co/lmz/candle-quantized-phi/resolve/main',
      model: 'model-q4k.gguf',
      tokenizer: 'tokenizer.json',
      config: 'phi-1_5.json',
      quantized: true,
      seq_len: 2048,
      size: '800 MB',
    },
  ],
  [
    'phi_1_5_q80',
    {
      base_url: 'https://huggingface.co/lmz/candle-quantized-phi/resolve/main',
      model: 'model-q80.gguf',
      tokenizer: 'tokenizer.json',
      config: 'phi-1_5.json',
      quantized: true,
      seq_len: 2048,
      size: '1.51 GB',
    },
  ],
  [
    'phi_2_0_q4k',
    {
      base_url: 'https://huggingface.co/radames/phi-2-quantized/resolve/main',
      model: ['model-v2-q4k.gguf_aa.part', 'model-v2-q4k.gguf_ab.part', 'model-v2-q4k.gguf_ac.part'],
      tokenizer: 'tokenizer.json',
      config: 'config.json',
      quantized: true,
      seq_len: 2048,
      size: '1.57GB',
    },
  ],
  [
    'puffin_phi_v2_q4k',
    {
      base_url: 'https://huggingface.co/lmz/candle-quantized-phi/resolve/main',
      model: 'model-puffin-phi-v2-q4k.gguf',
      tokenizer: 'tokenizer-puffin-phi-v2.json',
      config: 'puffin-phi-v2.json',
      quantized: true,
      seq_len: 2048,
      size: '798 MB',
    },
  ],
  [
    'puffin_phi_v2_q80',
    {
      base_url: 'https://huggingface.co/lmz/candle-quantized-phi/resolve/main',
      model: 'model-puffin-phi-v2-q80.gguf',
      tokenizer: 'tokenizer-puffin-phi-v2.json',
      config: 'puffin-phi-v2.json',
      quantized: true,
      seq_len: 2048,
      size: '1.50 GB',
    },
  ],
]);
