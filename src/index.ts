import debounce from 'lodash/debounce';
import init, { CompileContext } from 'markdown-render';
import './index.css';
import { MODELS } from './utils/models';
import { getStorage, setStorage } from './utils/storage';
import Worker from './worker?worker';

const textarea = document.querySelector('#container') as HTMLDivElement;
const inputTextarea = document.querySelector('#input-textarea') as HTMLTextAreaElement;
const lineNumbersTextarea = document.querySelector('#line-numbers-textarea') as HTMLTextAreaElement;
const modelSelect = document.querySelector('#model') as HTMLSelectElement;
const outputStatus = document.querySelector('#output-status') as HTMLElement;
const outputGeneratedText = document.querySelector('#background-text') as HTMLElement;
const outputCounter = document.querySelector('#output-counter') as HTMLElement;
const runBtn = document.querySelector('#run') as HTMLElement;
const renderBtn = document.querySelector('#render-result') as HTMLElement;
const llmWorker = new Worker();
let isRunning = false;
let runController = new AbortController();

const updateLineNumbers = (): void => {
  const lines = inputTextarea.value.split('\n');
  lineNumbersTextarea.value = lines.map((_, index) => (index + 1).toString()).join('\n');
};

const syncScroll = (): void => {
  lineNumbersTextarea.scrollTop = inputTextarea.scrollTop;
};

const startRunning = () => {
  isRunning = true;
  runBtn.textContent = 'Stop';
};

const stopRunning = () => {
  runController.abort();
  runController = new AbortController();
  runBtn.textContent = 'Run';
  isRunning = false;
};

const updateStatus = (data: any, content: string): void => {
  const { status, message, sentence, tokensSec, totalTime } = data;

  switch (status) {
    case 'loading':
      outputStatus.hidden = false;
      outputStatus.textContent = message;
      outputGeneratedText.hidden = true;
      outputCounter.hidden = true;
      break;
    case 'generating':
      outputStatus.hidden = true;
      outputCounter.hidden = false;
      outputGeneratedText.hidden = false;
      outputGeneratedText.innerHTML = `<pre class="whitespace-pre-wrap">${content}${sentence.replace(
        /<s>|<\/s>/g,
        '',
      )}</pre>`;
      outputCounter.innerHTML = `${(totalTime / 1000).toFixed(2)}s (${tokensSec.toFixed(2)} tok/s)`;
      break;
    case 'complete':
      outputStatus.hidden = true;
      outputGeneratedText.hidden = false;
      break;
  }
};

const generateSequence = (controller: AbortController): Promise<void> => {
  const getValue = (id: string): string => (document.querySelector(`#${id}`) as HTMLInputElement).value;
  const selectedModel = MODELS.get(getValue('model'));
  const content = getValue('input-textarea');
  const prompt = `${getValue('prompt')}
${content}`;

  llmWorker.postMessage({
    weightsURL: `${selectedModel?.base_url}/${selectedModel?.model}`,
    modelID: getValue('model'),
    configURL: `${selectedModel?.base_url}/${selectedModel?.config}`,
    tokenizerURL: `${selectedModel?.base_url}/${selectedModel?.tokenizer}`,
    prompt,
    temp: parseFloat(getValue('temperature')),
    top_p: parseFloat(getValue('top-p')),
    repeatPenalty: parseFloat(getValue('repeat_penalty')),
    seed: BigInt(getValue('seed')),
    maxSeqLen: parseInt(getValue('max-seq'), 10),
    command: 'start',
  });

  return new Promise((resolve, reject) => {
    const handleAbort = (): void => {
      llmWorker.postMessage({ command: 'abort' });
    };

    const handleMessage = (event: MessageEvent): void => {
      const { status, error } = event.data;
      if (status) updateStatus(event.data, content);
      if (error) {
        llmWorker.removeEventListener('message', handleMessage);
        reject(new Error(error));
      } else if (status === 'aborted' || status === 'complete') {
        llmWorker.removeEventListener('message', handleMessage);
        resolve(event.data);
      }
    };

    controller.signal.addEventListener('abort', handleAbort);
    llmWorker.addEventListener('message', handleMessage);
  });
};

const populateModelSelect = (): void => {
  MODELS.forEach((model, id) => {
    const option = document.createElement('option');
    option.value = id;
    option.innerText = `${id} (${model.size})`;
    modelSelect.appendChild(option);
  });

  const query = new URLSearchParams(window.location.search);
  modelSelect.value = query.get('model') || 'phi_1_5_q4k';
};

const loadStoredInput = (): void => {
  const storedUserInput = getStorage('userInput');
  if (storedUserInput) {
    inputTextarea.value = storedUserInput;
    updateLineNumbers();
  }
};

const handleTextareaUpdate = (): void => {
  const backgroundText = document.querySelector('#background-text')!.innerText;
  if (inputTextarea.value.length > backgroundText.length) return;
  inputTextarea.value = backgroundText;
  inputTextarea.dispatchEvent(new Event('input'));
};

const renderInput = async () => {
  await init();
  const context = CompileContext.new();

  inputTextarea.addEventListener(
    'input',
    debounce(async (e: Event) => {
      const target = e.target as HTMLTextAreaElement;
      context.render(target.value);
      setStorage('userInput', target.value, 500);
      syncScroll();
    }, 200),
  );

  inputTextarea.dispatchEvent(new Event('input'));
};

runBtn.addEventListener('click', async () => {
  if (isRunning) {
    stopRunning();
  } else {
    startRunning();
    await generateSequence(runController);
    stopRunning();
  }
});

document.addEventListener('DOMContentLoaded', () => {
  populateModelSelect();
  loadStoredInput();
});

renderBtn.addEventListener('click', () => {
  handleTextareaUpdate();
  outputGeneratedText.innerHTML = '';
});

textarea.addEventListener('input', updateLineNumbers);

window.addEventListener('load', () => {
  loadStoredInput();
});

void renderInput();
