// import init, { render_markdown, t4, wwwwaaaa } from 'my-crate'
// import remarkParse from 'remark-parse'
// import { unified } from 'unified'
// import './index.css'

// async function run(markdownText) {
//   const res = await init()
//   const encoder = new TextEncoder()
//   const stringBytes = encoder.encode(markdownText)
//   const iterations = 1
//   // const htmlResultJS = marked(markdownText);
//   const mdast = unified().use(remarkParse).parse(markdownText)
//   const htmlResultWasm = render_markdown(markdownText)
//   const jsStartTime = performance.now()
//   for (let i = 0; i < iterations; i++) {
//     // Perform JavaScript Markdown rendering
//     const mdast = unified().use(remarkParse).parse(markdownText)
//     // console.log(htmlResultJS);
//   }
//   const jsEndTime = performance.now()
//   const jsTime = jsEndTime - jsStartTime
//   console.log(`JavaScript time: ${jsTime} ms`)

//   // Benchmark WebAssembly
//   const wasmStartTime = performance.now()
//   // const ptr = new_buffer('test', markdownText.length);
//   // const u8Arr = new Uint8ClampedArray(res.memory.buffer, ptr, markdownText.length);
//   // u8Arr.set(stringBytes);
//   // const ws = render_markdown('test');

//   for (let i = 0; i < iterations; i++) {
//     // Perform WebAssembly Markdown rendering
//     t4(markdownText)
//   }
//   const wasmEndTime = performance.now()
//   const wasmTime = wasmEndTime - wasmStartTime
//   console.log(`WebAssembly time: ${wasmTime} ms`)
//   const jsStartTime1 = performance.now()
//   for (let i = 0; i < iterations; i++) {
//     // Perform JavaScript Markdown rendering
//     const dom = document.getElementById('test1')
//     dom && (dom.innerText = 'Rust 操作 Dom')
//     // console.log(htmlResultJS);
//   }
//   const jsEndTime1 = performance.now()
//   const jsTime1 = jsEndTime1 - jsStartTime1
//   console.log(`JavaScript time: ${jsTime1} ms`)
//   const wasmStartTime1 = performance.now()
//   // const ptr = new_buffer('test', markdownText.length);
//   // const u8Arr = new Uint8ClampedArray(res.memory.buffer, ptr, markdownText.length);
//   // u8Arr.set(stringBytes);
//   // const ws = render_markdown('test');

//   for (let i = 0; i < iterations; i++) {
//     // Perform WebAssembly Markdown rendering
//     const htmlResultWasm = wwwwaaaa()
//   }
//   const wasmEndTime1 = performance.now()
//   const wasmTime1 = wasmEndTime1 - wasmStartTime1
//   // console.log(`WebAssembly time: ${wasmTime1} ms`);

//   // 将 HTML 结果插入到页面中
// }

// const complexMarkdownString = `
// # Complex Markdown Example

// ## Lists

// 1. First item
// 2. Second item
//    - Nested item
//    - Another nested item
// 3. Third item

// ## Code Block

// \`\`\`javascript
// function greet(name) {
//   console.log("Hello, " + name + "!");
// }
// \`\`\`

// ## Table

// | Name  | Age | Occupation       |
// |-------|-----|------------------|
// | Alice | 28  | Software Engineer|
// | Bob   | 35  | Data Scientist   |

// ## Links

// [Google](https://www.google.com/)
// [GitHub](https://github.com/)

// ## Emphasis

// *Italic text*
// **Bold text**

// ## Blockquote

// > This is a blockquote.

// ## Images

// ![Markdown Logo](https://markdown-here.com/img/icon256.png)

// ## Horizontal Rule

// ---

// ## Task List

// - [x] Task 1
// - [ ] Task 2
// - [x] Task 3

// ## Footnotes

// Here is some text with a footnote[^1].

// [^1]: This is the footnote content.

// ## Headers with IDs

// ### Header 1 {#header-1}
// Some content under Header 1.

// ### Header 2 {#header-2}
// Some content under Header 2.

// ## Definition List

// Markdown
// : Lightweight markup language.
// `

// const md = `# Heading 1

// ## Heading 2

// as*a*dasd
// as**d**asd
// <div>aaa</div>

// 1. List item 1
// 2. List item 2

// \`\`\`rust
// fn main() {
//     println!("Hello, World!");
// }`

// const md1 = `# Hello
// | abc | def |
// | --- | --- |
// | bar | baz |`

// const md2 = `111
// 222\`aaa\`
// <div>
// aaa
// </div>

// [OpenAI](https://www.openai.com/)
// ![OpenAI](https://www.openai.com/)

//         `

// run(md2)
// // run("# Hello, World123333!\n".repeat(1));
// // run("# Hello, World41222222222222312233333333333!\n".repeat(1));

// const inputTextarea = document.getElementById('input-textarea') as HTMLTextAreaElement
// const lineNumbersTextarea = document.getElementById('line-numbers-textarea') as HTMLTextAreaElement
// const textarea = document.getElementById('container') as HTMLDivElement
// const updateLineNumbers = () => {
//   if (inputTextarea) {
//     const lines = inputTextarea.value.split('\n')
//     let lineNumbers = ''
//     for (let i = 1; i <= lines.length; i++) {
//       lineNumbers += i + '\n'
//     }

//     lineNumbersTextarea.value = lineNumbers.trim()
//   }
// }

// const syncScroll = () => {
//   lineNumbersTextarea.scrollTop = inputTextarea.scrollTop
//   inputTextarea.style.height = `${lineNumbersTextarea.scrollHeight}px`
// }

// textarea.oninput = updateLineNumbers
// inputTextarea.oninput = syncScroll
