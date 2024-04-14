import init, { CompileContext } from 'my-crate'
import './index.css'
import { getStorage, setStorage } from './utils/storage'

const inputTextarea = document.getElementById('input-textarea') as HTMLTextAreaElement
const lineNumbersTextarea = document.getElementById('line-numbers-textarea') as HTMLTextAreaElement
const textarea = document.getElementById('container') as HTMLDivElement
const updateLineNumbers = () => {
  if (inputTextarea) {
    const lines = inputTextarea.value.split('\n')
    let lineNumbers = ''
    for (let i = 1; i <= lines.length; i++) {
      lineNumbers += i.toString() + '\n'
    }

    lineNumbersTextarea.value = lineNumbers.trim()
  }
}
window.addEventListener('load', () => {
  const storedUserInput = getStorage('userInput')
  if (storedUserInput) {
    inputTextarea.value = storedUserInput
    updateLineNumbers()
  }
})

textarea.addEventListener('input', updateLineNumbers)

const syncScroll = () => {
  lineNumbersTextarea.scrollTop = inputTextarea.scrollTop
  inputTextarea.style.height = `${lineNumbersTextarea.scrollHeight}px`
}

async function main() {
  await init()
  const context = CompileContext.new()

  const handleInput = (e: InputEvent) => {
    const target = e.target as HTMLTextAreaElement
    context.render(target.value)
    setStorage('userInput', target.value, 500)
  }

  inputTextarea.addEventListener('input', (e: Event) => {
    syncScroll()
    void handleInput(e as InputEvent)
  })

  const inputEvent = new Event('input')
  inputTextarea.dispatchEvent(inputEvent)
}

void main()
