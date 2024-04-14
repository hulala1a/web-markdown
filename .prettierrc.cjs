module.exports = {
  printWidth: 120, // 单行宽度限制
  tabWidth: 2, // tab 使用两个空格
  useTabs: false, // 不使用制表符缩进，使用空格缩进
  semi: false, // 代码不需要分号结尾
  singleQuote: true, // 单引号
  bracketSpacing: true, // 对象左右两侧需要空格
  // jsxSingleQuote: true, //  在 JSX 中使用单引号代替双引号
  trailingComma: 'all', //  尽可能尾随逗号
  endOfLine: 'lf',
  // 'insertPragma': true,
  // jsxBracketSameLine: false, // html 关闭标签换行
  arrowParens: 'avoid', // 单参数的箭头函数参数不需要括号
  proseWrap: 'never', // 参考 https://prettier.io/docs/en/options.html#prose-wrap
}
