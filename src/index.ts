//@ts-ignore
import init, { render_markdown,new_buffer,wwwwaaaa,t } from 'my-crate';
import {marked} from 'marked';
import {unified} from 'unified';
import remarkParse from 'remark-parse';
import stringify from 'remark-stringify';
import inspect from 'unist-util-inspect';

// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

async function run(markdownText) {
  const res = await init();
  const encoder = new TextEncoder();
  const stringBytes = encoder.encode(markdownText); 
  const iterations = 10000;
  // const htmlResultJS = marked(markdownText);
  const mdast = unified().use(remarkParse).parse(markdownText);
  const htmlResultWasm = render_markdown(markdownText);
  const jsStartTime = performance.now();
  for (let i = 0; i < iterations; i++) {
    // Perform JavaScript Markdown rendering
    const mdast = unified().use(remarkParse).parse(markdownText);
    // console.log(htmlResultJS);

  }
  const jsEndTime = performance.now();
  const jsTime = jsEndTime - jsStartTime;
  console.log(`JavaScript time: ${jsTime} ms`);

  // Benchmark WebAssembly
  const wasmStartTime = performance.now();
  // const ptr = new_buffer('test', markdownText.length);
  // const u8Arr = new Uint8ClampedArray(res.memory.buffer, ptr, markdownText.length);
  // u8Arr.set(stringBytes);
  // const ws = render_markdown('test');

  for (let i = 0; i < iterations; i++) {

    // Perform WebAssembly Markdown rendering
    t(markdownText);
    
  }
  const wasmEndTime = performance.now();
  const wasmTime = wasmEndTime - wasmStartTime;
  console.log(`WebAssembly time: ${wasmTime} ms`);
  const jsStartTime1 = performance.now();
  for (let i = 0; i < iterations; i++) {
    // Perform JavaScript Markdown rendering
    const dom = document.getElementById('test1');
    dom&&(dom.innerText = "Rust 操作 Dom")
    // console.log(htmlResultJS);

  }
  const jsEndTime1 = performance.now();
  const jsTime1 = jsEndTime1 - jsStartTime1;
  console.log(`JavaScript time: ${jsTime1} ms`);
  const wasmStartTime1 = performance.now();
  // const ptr = new_buffer('test', markdownText.length);
  // const u8Arr = new Uint8ClampedArray(res.memory.buffer, ptr, markdownText.length);
  // u8Arr.set(stringBytes);
  // const ws = render_markdown('test');

  for (let i = 0; i < iterations; i++) {

    // Perform WebAssembly Markdown rendering
    const htmlResultWasm = wwwwaaaa();
  }
  const wasmEndTime1 = performance.now();
  const wasmTime1 = wasmEndTime1 - wasmStartTime1;
  console.log(`WebAssembly time: ${wasmTime1} ms`);

  // 将 HTML 结果插入到页面中
}

const complexMarkdownString = `
# Complex Markdown Example

## Lists

1. First item
2. Second item
   - Nested item
   - Another nested item
3. Third item

## Code Block

\`\`\`javascript
function greet(name) {
  console.log("Hello, " + name + "!");
}
\`\`\`

## Table

| Name  | Age | Occupation       |
|-------|-----|------------------|
| Alice | 28  | Software Engineer|
| Bob   | 35  | Data Scientist   |

## Links

[Google](https://www.google.com/)
[GitHub](https://github.com/)

## Emphasis

*Italic text*
**Bold text**

## Blockquote

> This is a blockquote.

## Images

![Markdown Logo](https://markdown-here.com/img/icon256.png)

## Horizontal Rule

---

## Task List

- [x] Task 1
- [ ] Task 2
- [x] Task 3

## Footnotes

Here is some text with a footnote[^1].

[^1]: This is the footnote content.

## Headers with IDs

### Header 1 {#header-1}
Some content under Header 1.

### Header 2 {#header-2}
Some content under Header 2.

## Definition List

Markdown
: Lightweight markup language.
`;

run(complexMarkdownString.repeat(1));
// run("# Hello, World123333!\n".repeat(1));
// run("# Hello, World41222222222222312233333333333!\n".repeat(1));
