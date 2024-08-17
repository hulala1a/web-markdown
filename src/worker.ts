import init, { Model } from 'text-generation';

interface GenerateData {
  command: string;
  weightsURL: string;
  configURL: string;
  modelID: string;
  tokenizerURL: string;
  prompt: string;
  temp: number;
  top_p: number;
  repeatPenalty: number;
  seed: bigint;
  maxSeqLen?: number;
}

class TextGeneration {
  static instance: { [key: string]: Model } = {};

  static getInstance = async (
    weightsURL: string | string[],
    modelID: string,
    tokenizerURL: string,
    configURL: string,
  ): Promise<Model> => {
    // Load individual modelID only once
    if (!this.instance[modelID]) {
      await init();

      self.postMessage({ status: 'loading', message: 'Loading Model' });

      const [weightsArrayU8, tokenizerArrayU8, configArrayU8] = await Promise.all([
        weightsURL instanceof Array ? concatenateArrayBuffers(weightsURL) : fetchArrayBuffer(weightsURL),
        fetchArrayBuffer(tokenizerURL),
        fetchArrayBuffer(configURL),
      ]);
      console.log(weightsArrayU8);
      this.instance[modelID] = new Model(weightsArrayU8, tokenizerArrayU8, configArrayU8);
    }
    return this.instance[modelID];
  };
}

const fetchArrayBuffer = async (url: string): Promise<Uint8Array> => {
  const cacheName = 'llm-cache';
  const cache = await caches.open(cacheName);
  const cachedResponse = await cache.match(url);
  if (cachedResponse) {
    const data = await cachedResponse.arrayBuffer();
    return new Uint8Array(data);
  }
  const token = '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';
  const res = await fetch(url, {
    cache: 'force-cache',
  });
  cache.put(url, res.clone());
  return new Uint8Array(await res.arrayBuffer());
};

async function concatenateArrayBuffers(urls: string[]) {
  const arrayBuffers = await Promise.all(urls.map(url => fetchArrayBuffer(url)));

  let totalLength = arrayBuffers.reduce((acc, arrayBuffer) => acc + arrayBuffer.byteLength, 0);
  let concatenatedBuffer = new Uint8Array(totalLength);

  let offset = 0;
  arrayBuffers.forEach(buffer => {
    concatenatedBuffer.set(new Uint8Array(buffer), offset);
    offset += buffer.byteLength;
  });
  return concatenatedBuffer;
}

const generate = async (data: GenerateData) => {
  const { weightsURL, modelID, tokenizerURL, prompt, temp, top_p, repeatPenalty, seed, maxSeqLen, configURL } = data;
  try {
    self.postMessage({ status: 'loading', message: 'Starting llama2.c' });
    const model = await TextGeneration.getInstance(weightsURL, modelID, tokenizerURL, configURL);

    self.postMessage({ status: 'loading', message: 'Initializing model' });
    const firstToken = model.init_with_prompt(prompt, temp, top_p, repeatPenalty, seed);

    let sentence = firstToken;
    let maxTokens = maxSeqLen || 0;
    let startTime = performance.now();
    let tokensCount = 0;

    while (tokensCount < maxTokens) {
      await new Promise<void>(async resolve => {
        if (controller && controller.signal.aborted) {
          self.postMessage({
            status: 'aborted',
            message: 'Aborted',
            output: prompt + sentence,
          });
          return;
        }
        const token = await model.next_token();
        const tokensSec = ((tokensCount + 1) / (performance.now() - startTime)) * 1000;

        sentence += token;
        self.postMessage({
          status: 'generating',
          message: 'Generating token',
          token: token,
          sentence: sentence,
          totalTime: performance.now() - startTime,
          tokensSec,
          prompt: prompt,
        });
        setTimeout(resolve, 0);
      });
      tokensCount++;
    }
    self.postMessage({
      status: 'complete',
      message: 'complete',
      output: prompt + sentence,
    });
  } catch (e) {
    self.postMessage({ error: e });
  }
};

let controller: AbortController | null = null;

self.addEventListener('message', (event: MessageEvent<GenerateData>) => {
  if (event.data.command === 'start') {
    controller = new AbortController();
    console.log(event.data);
    generate(event.data);
  } else if (event.data.command === 'abort') {
    console.log('abort');
    controller?.abort();
  }
});
